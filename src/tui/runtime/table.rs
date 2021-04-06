use crate::{
    prelude::*,
    tui::{layout::full_win_layout, prelude::*, runtime::StatefulEventHandler, widgets::*},
};
use crossterm::{
    event::{
        DisableMouseCapture, EnableMouseCapture, Event as CrossEvent, KeyCode, KeyEvent,
        KeyModifiers,
    },
    execute,
    terminal::*,
};
use tui::{backend::CrosstermBackend, Terminal};

pub struct TableExitStatus {
    pub go_request: Option<String>,
    pub new_cmd: Option<popup::add::AddCmdUi>,
    pub rm_selection: Vec<String>,
    pub success: bool,
}

impl StatefulCmdsTable<'_> {
    const DFL_STATE: u8 = 0;
    const ADD_STATE: u8 = 1;
    const EDIT_STATE: u8 = 2;
    const RM_STATE: u8 = 3;
    const EXIT_STATE: u8 = 4;

    fn go_handler(&self, exit_status: &mut TableExitStatus, selected_index: usize) {
        let key = self
            .cmds
            .borrow()
            .keys()
            .nth(selected_index)
            .unwrap()
            .to_string();
        exit_status.go_request.replace(key);
        exit_status.success = true;
    }

    fn rm_handler(&mut self, exit_status: &mut TableExitStatus, selected_index: usize) {
        let key = self
            .cmds
            .borrow()
            .keys()
            .nth(selected_index)
            .unwrap()
            .to_string();
        exit_status.rm_selection.push(key);
        self.selected_indices.push(selected_index);
        exit_status.success = true;
    }

    fn state_handler(t: u8) -> StatefulEventHandler {
        match t {
            Self::DFL_STATE | Self::EXIT_STATE => StatefulEventHandler::new(),
            Self::ADD_STATE => StatefulEventHandler::for_add_popup(),
            Self::EDIT_STATE => StatefulEventHandler::for_edit_popup(),
            Self::RM_STATE => StatefulEventHandler::for_rm_popup(Default::default()),
            _ => panic!("Not a valid tui state"),
        }
    }

    pub fn render(&mut self) -> Result<TableExitStatus> {
        // app state
        let exit_requested = &mut false;
        let mut exit_status = TableExitStatus {
            new_cmd: None,
            go_request: None,
            rm_selection: vec![],
            success: false,
        };
        let exit_status_ref = RefCell::from(&mut exit_status);
        let ui_state = &mut 0_u8;

        // setup for crossterm env
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // spawn the event loop
        let rx = Event::spawn_loop(Event::POLL_RATE);
        loop {
            let StatefulEventHandler {
                ref mut state,
                ref handler,
            } = Self::state_handler(*ui_state);

            terminal.draw(|frame| {
                let layout = full_win_layout(frame);
                self.show_layout(frame, layout);
                state.render(frame);
            })?;

            let event = rx.recv()?;
            if let Some((column, row)) = event.clicked_coords() {
                terminal.set_cursor(column, row)?;
                self.select(row as usize - 2);
            }

            let mut request_popup_close = false;

            match state {
                PopupState::Add(ref mut seq) => {
                    // bindings while add popup is open
                    match event {
                        a if handler.accepts(&a) => seq.push(),
                        r if handler.rejects(&r) => request_popup_close = true,
                        Event(CrossEvent::Key(KeyEvent {
                            code,
                            modifiers: KeyModifiers::NONE,
                        })) => match code {
                            KeyCode::Backspace | KeyCode::Delete => seq.delete(),
                            KeyCode::Left => seq.pop(),
                            KeyCode::Char(c) => seq.print(c),
                            _ => {}
                        },
                        _ => {}
                    }

                    if seq.done() {
                        let (key, cmd) = seq.generate()?;
                        (*(*self.cmds.borrow_mut())).insert(key, cmd);
                        request_popup_close = true;
                    }
                }
                PopupState::Edit => {}
                PopupState::ExitInfo(_) => {
                    std::thread::sleep(std::time::Duration::from_secs(7));

                    disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;
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
                    add if add == EventHandler::ADD => *ui_state = Self::ADD_STATE,
                    edit if edit == EventHandler::EDIT => *ui_state = Self::EDIT_STATE,
                    go if go == EventHandler::GO || go == '\n' => {
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
                *ui_state = Self::DFL_STATE;
            }

            if *exit_requested {
                if !exit_status_ref.borrow().rm_selection.is_empty() {
                    *ui_state = Self::RM_STATE;
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
