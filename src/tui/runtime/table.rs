use crate::{
    prelude::*,
    tui::{
        layout::full_win_layout,
        prelude::*,
        runtime::events::{event_handlers, StatefulEventHandler},
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
use tui::{backend::CrosstermBackend, Terminal};

#[derive(Debug)]
pub struct TableExitStatus {
    pub go_request: Option<String>,
    pub rm_selection: Vec<String>,
    pub success: bool,
}

impl StatefulCmdsTable<'_> {
    pub const DFL_STATE: usize = 0;
    pub const ADD_STATE: usize = 1;
    pub const EDIT_STATE: usize = 2;
    pub const RM_STATE: usize = 3;
    pub const EXIT_STATE: usize = 4;

    fn cmd_key_for_index(&self, index: &usize) -> String {
        self.key_cache
            .get(index)
            .seppuku("UNEXPECTED OUT OF BOUNDS CACHE ACCESS")
            .to_string()
    }

    fn go_handler(&self, exit_status: &mut TableExitStatus, selected_index: usize) {
        let key = self.cmd_key_for_index(&selected_index);
        exit_status.go_request.replace(key);
        exit_status.success = true;
    }

    fn rm_handler(&mut self, exit_status: &mut TableExitStatus, selected_index: usize) {
        let key = self.cmd_key_for_index(&selected_index);
        exit_status.rm_selection.push(key);
        self.selected_indices.push(selected_index);
        exit_status.success = true;
    }

    pub fn render(
        &mut self,
        testing_poller: Option<&std::sync::mpsc::Sender<usize>>,
    ) -> Result<TableExitStatus> {
        // app state
        let exit_requested = &mut false;
        let request_popup_close = &mut false;
        let popup_context: &mut Option<String> = &mut None;
        let mut exit_status = TableExitStatus {
            go_request: None,
            rm_selection: vec![],
            success: false,
        };
        let exit_status_ref = RefCell::from(&mut exit_status);
        let ui_state = &mut 0_usize;

        let mut event_handlers = event_handlers();
        // setup for crossterm env
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // spawn the event loop
        let rx = Event::spawn_loop(Event::POLL_RATE);
        loop {
            if let Some(sender) = testing_poller {
                sender.send(*ui_state).seppuku(None);
            }

            let StatefulEventHandler {
                ref mut state,
                ref handler,
            } = event_handlers[*ui_state];

            terminal.draw(|frame| {
                let layout = full_win_layout(frame);
                self.show_layout(frame, layout);
                state.render(frame, popup_context);
            })?;

            if *ui_state == Self::EXIT_STATE {
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

            let event = rx.recv()?;
            if let Some((column, row)) = event.clicked_coords() {
                terminal.set_cursor(column, row)?;
                self.select(row as usize - 2);
            }

            match state {
                PopupState::Add(ref mut seq) => {
                    // bindings while add popup is open
                    match event {
                        a if handler.accepts(&a) => seq.try_push()?,
                        r if handler.rejects(&r) => *request_popup_close = true,
                        Event(CrossEvent::Key(KeyEvent {
                            code,
                            modifiers: KeyModifiers::NONE,
                        })) => match code {
                            KeyCode::Backspace | KeyCode::Delete => seq.delete(),
                            KeyCode::Left => seq.pop(),
                            KeyCode::Char(c) => seq.print(c),
                            _ => {}
                        },
                        Event(CrossEvent::Key(KeyEvent {
                            code: KeyCode::Char(c),
                            modifiers: KeyModifiers::SHIFT,
                        })) => seq.print(c.to_ascii_uppercase()),
                        _ => {}
                    }

                    if seq.done() {
                        let (key, cmd) = seq.into_cmd()?;
                        (*(*self.cmds.borrow_mut())).insert(key.clone(), cmd);
                        self.update_cache();
                        popup_context.replace(format!("{} was added", key));
                        continue;
                    }
                }
                PopupState::Edit(ref mut seq) => {
                    use crate::tui::widgets::popup::edit::EditSeq;

                    match event {
                        a if handler.accepts(&a) => seq.try_push()?,
                        r if handler.rejects(&r) => *request_popup_close = true,
                        Event(CrossEvent::Key(KeyEvent {
                            code,
                            modifiers: KeyModifiers::NONE,
                        })) => match code {
                            KeyCode::Backspace | KeyCode::Delete => seq.delete(),
                            KeyCode::Left => seq.pop(),
                            KeyCode::Char(c) => seq.print(c),
                            _ => {}
                        },
                        Event(CrossEvent::Key(KeyEvent {
                            code: KeyCode::Char(c),
                            modifiers: KeyModifiers::SHIFT,
                        })) => seq.print(c.to_ascii_uppercase()),
                        _ => {}
                    }

                    let SeqFrame { ref query, .. } = seq.current_frame();
                    let cmd_key = self.cmd_key_for_index(&self.state.selected().unwrap());
                    if let Some(ref mut curr_cmd) = (*(*self.cmds.borrow_mut())).get_mut(&cmd_key) {
                        EditSeq::set_new_val(&query, &mut seq.buf, curr_cmd)
                            .map_err(|e| anyhow!(e))?;
                    }
                    if seq.done() {
                        self.update_cache();
                        *request_popup_close = true;
                    }
                }
                PopupState::ExitWithMsg | PopupState::Info => unreachable!(),
                PopupState::RmConfirm => match event {
                    a if handler.accepts(&a) => {
                        exit_status_ref.borrow_mut().success = true;
                        popup_context.replace(format!(
                            "{} removed!",
                            exit_status_ref.borrow().rm_selection.join(", ")
                        ));
                        *ui_state = Self::EXIT_STATE;
                    }
                    r if handler.rejects(&r) => {
                        exit_status_ref.borrow_mut().success = false;
                        popup_context
                            .replace("Operation cancelled. Commands will not be removed.".into());
                        *exit_requested = true;
                    }
                    _ => {
                        popup_context.replace("(y)es or (n)o?".into());
                        continue;
                    }
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

                        if !exit_status_ref.borrow().rm_selection.is_empty() {
                            popup_context.replace(format!(
                                "Remove {}?",
                                exit_status_ref.borrow().rm_selection.join(", ")
                            ));
                            *ui_state = Self::RM_STATE;
                            continue;
                        }

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

            if *request_popup_close {
                *ui_state = Self::DFL_STATE;
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
