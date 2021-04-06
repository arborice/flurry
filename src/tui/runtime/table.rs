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
    event::{
        DisableMouseCapture, EnableMouseCapture, Event as CrossEvent, KeyCode, KeyEvent,
        KeyModifiers,
    },
    execute,
    terminal::*,
};
use parking_lot::Mutex;
use tui::{backend::CrosstermBackend, Terminal};

pub struct TableExitStatus {
    pub go_request: Option<String>,
    pub last_requested_action: Option<char>,
    pub new_cmd: Option<popup::add::AddCmdUi>,
    pub rm_selection: Vec<String>,
    pub success: bool,
}

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

    pub fn render(&mut self) -> Result<TableExitStatus> {
        // app state
        let exit_requested = &mut false;
        let mut exit_status = TableExitStatus {
            new_cmd: None,
            go_request: None,
            last_requested_action: None,
            rm_selection: vec![],
            success: false,
        };
        let exit_status_ref = RefCell::from(&mut exit_status);
        let event_handler = StatefulEventHandler::new();
        let ui_state = RefCell::from(Mutex::from(event_handler));

        // setup for crossterm env
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // spawn the event loop
        let rx = Event::spawn_loop(Event::POLL_RATE);
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
            if let Some((column, row)) = event.clicked_coords() {
                terminal.set_cursor(column, row)?;
                self.select(row as usize - 2);
            }

            let mut request_popup_close = false;
            let ui_guard = ui_state.borrow_mut();
            let StatefulEventHandler {
                ref mut state,
                ref handler,
            } = *ui_guard.lock();

            match state {
                PopupState::Add(ref mut seq) => {
                    match event {
                        a if handler.accepts(&a) => seq.push(),
                        r if handler.rejects(&r) => request_popup_close = true,
                        Event(CrossEvent::Key(KeyEvent {
                            code,
                            modifiers: KeyModifiers::NONE,
                        })) => match code {
                            KeyCode::Backspace => seq.delete(),
                            KeyCode::Left => seq.pop(),
                            KeyCode::Char(c) => seq.print(c),
                            _ => {}
                        },
                        _ => {}
                    }

                    if seq.done() {
                        let mut status = exit_status_ref.borrow_mut();
                        seq.hydrate(&mut status.new_cmd);
                        status.success = true;
                        request_popup_close = true;
                    }
                }
                PopupState::Edit => {}
                PopupState::ExitInfo(_) => {
                    std::thread::sleep(std::time::Duration::from_secs(7));
                    break;
                }
                PopupState::RmConfirm(_) => match event {
                    a if handler.accepts(&a) => {
                        exit_status_ref.borrow_mut().success = true;
                        *exit_requested = true;
                    }
                    r if handler.rejects(&r) => {
                        exit_status_ref.borrow_mut().success = false;
                        *state = PopupState::ExitInfo("Operation cancelled. Commands NOT removed.");
                    }
                    _ => {}
                },
                PopupState::Closed => match event {
                    add if add == EventHandler::ADD => {
                        *ui_guard.lock() = StatefulEventHandler::for_add_popup();
                        exit_status_ref
                            .borrow_mut()
                            .last_requested_action
                            .replace(EventHandler::ADD);
                    }
                    edit if edit == EventHandler::EDIT => {
                        *ui_guard.lock() = StatefulEventHandler::for_edit_popup();
                    }
                    go if go == EventHandler::GO => {
                        if let Some(selected_index) = self.state.selected() {
                            self.go_handler(&mut exit_status_ref.borrow_mut(), selected_index);
                            *exit_requested = true;
                        }
                    }
                    rm if rm == EventHandler::RM => {
                        if let Some(selected_index) = self.state.selected() {
                            self.rm_handler(&mut exit_status_ref.borrow_mut(), selected_index);
                        }
                    }
                    a if handler.accepts(&a) => {
                        exit_status_ref.borrow_mut().success = true;
                        *exit_requested = true;
                    }
                    r if handler.rejects(&r) => {
                        exit_status_ref.borrow_mut().success = false;
                        *exit_requested = true;
                    }
                    ev if ev.is_next_trigger() => self.next(),
                    ev if ev.is_prev_trigger() => self.previous(),
                    _ => {}
                },
            }

            if request_popup_close {
                *ui_guard.lock() = StatefulEventHandler::new();
            }

            if *exit_requested {
                if !exit_status_ref.borrow().rm_selection.is_empty() {
                    let selection = exit_status_ref.borrow().rm_selection.join(", ");
                    *ui_guard.lock() = StatefulEventHandler::for_rm_popup(selection);
                    continue;
                }

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
