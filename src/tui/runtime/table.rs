use crate::{
    prelude::*,
    tui::{
        layout::{full_win_layout, table_layout},
        prelude::*,
        runtime::StatefulEventHandler,
        widgets::*,
    },
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::*,
};

use popup::add::AddSequence;
use tui::{backend::CrosstermBackend, Terminal};

pub struct TableExitStatus {
    pub go_request: Option<String>,
    pub last_requested_action: Option<char>,
    pub new_cmd: Option<popup::add::AddCmdUi>,
    pub rm_selection: Vec<String>,
    pub success: bool,
}

use parking_lot::Mutex;

impl StatefulCmdsTable<'_> {
    fn go_handler(&self, exit_status: &mut TableExitStatus, selected_index: usize) {
        let key = self.cmds.borrow()[selected_index].0.to_string();
        exit_status.last_requested_action.replace(EventHandler::GO);
        exit_status.go_request.replace(key);
        exit_status.success = true;
    }

    fn rm_handler(&mut self, exit_status: &mut TableExitStatus, selected_index: usize) {
        let key = self.cmds.borrow()[selected_index].0.to_string();
        exit_status.rm_selection.push(key);
        self.selected_indices.push(selected_index);
        exit_status.success = true;
    }

    pub fn render(&mut self, event_handler: StatefulEventHandler) -> Result<TableExitStatus> {
        // app state
        let exit_requested = &mut false;
        let mut exit_status = TableExitStatus {
            new_cmd: None,
            go_request: None,
            last_requested_action: None,
            rm_selection: vec![],
            success: false,
        };
        let exit_status_ref = &mut exit_status;

        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let rx = spawn_event_loop();

        let ui_state: RefCell<Mutex<StatefulEventHandler>> =
            RefCell::from(Mutex::from(event_handler));

        loop {
            terminal.draw(|frame| {
                let layout = full_win_layout(frame);
                let _table = table_layout(self, frame, layout, Default::default());

                {
                    let ui_guard = ui_state.borrow_mut();
                    let ref popup_state = (*ui_guard.lock()).state;
                    popup_state.render(frame);
                }
            })?;

            let event = rx.recv()?;
            if let Some((column, row)) = Event::clicked_coords(&event) {
                terminal.set_cursor(column, row)?;
                self.select(row as usize - 2);
            }

            {
                let mut request_popup_close = false;
                let ui_guard = ui_state.borrow_mut();
                let StatefulEventHandler {
                    ref mut state,
                    ref handler,
                } = *ui_guard.lock();

                match state {
                    PopupState::Add(ref mut seq) => {
                        use crossterm::event::{
                            Event as CrossEvent, KeyCode, KeyEvent, KeyModifiers,
                        };
                        match event {
                            a if handler.accepts(&a) => seq.push(),
                            r if handler.rejects(&r) => request_popup_close = true,
                            CrossEvent::Key(KeyEvent {
                                code,
                                modifiers: KeyModifiers::NONE,
                            }) => match code {
                                KeyCode::Backspace => seq.delete(),
                                KeyCode::Left => seq.pop(),
                                KeyCode::Char(c) => seq.print(c),
                                _ => {}
                            },
                            _ => {}
                        }

                        if seq.done() {
                            seq.hydrate(&mut exit_status_ref.new_cmd);
                            exit_status_ref.success = true;
                            *state = PopupState::Closed;
                        }
                    }
                    PopupState::Edit => {}
                    PopupState::ExitInfo(_) => {
                        std::thread::sleep(std::time::Duration::from_secs(7));
                        break;
                    }
                    PopupState::RmConfirm(_) => match event {
                        a if handler.accepts(&a) => {
                            exit_status_ref.success = true;
                            *exit_requested = true;
                        }
                        r if handler.rejects(&r) => {
                            exit_status_ref.success = false;
                            *state =
                                PopupState::ExitInfo("Operation cancelled. Commands NOT removed.");
                        }
                        _ => {}
                    },
                    PopupState::Closed => match event {
                        add if Event(add) == EventHandler::ADD => {
                            *state = PopupState::Add(AddSequence::new());
                            exit_status_ref
                                .last_requested_action
                                .replace(EventHandler::ADD);
                        }
                        edit if Event(edit) == EventHandler::EDIT => {
                            *state = PopupState::Edit;
                        }
                        go if Event(go) == EventHandler::GO => {
                            if let Some(selected_index) = self.state.selected() {
                                self.go_handler(exit_status_ref, selected_index);
                                *exit_requested = true;
                            }
                        }
                        rm if Event(rm) == EventHandler::RM => {
                            if let Some(selected_index) = self.state.selected() {
                                self.rm_handler(exit_status_ref, selected_index);
                            }
                        }
                        a if handler.accepts(&a) => {
                            exit_status_ref.success = true;
                            *exit_requested = true;
                        }
                        r if handler.rejects(&r) => {
                            exit_status_ref.success = false;
                            *exit_requested = true;
                        }
                        ev if Event::is_next_trigger(&ev) => self.next(),
                        ev if Event::is_prev_trigger(&ev) => self.previous(),
                        _ => {}
                    },
                }

                if request_popup_close {
                    *state = PopupState::Closed;
                }
            }

            if *exit_requested {
                disable_raw_mode()?;
                execute!(
                    terminal.backend_mut(),
                    LeaveAlternateScreen,
                    DisableMouseCapture
                )?;
                terminal.show_cursor()?;
                break;
            }
        }
        Ok(exit_status)
    }
}
