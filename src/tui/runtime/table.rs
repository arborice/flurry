use super::*;
use crate::{
    prelude::*,
    tui::{
        layouts::{full_win_layout, table_layout},
        prelude::*,
        widgets::popup::render_popup,
    },
};
use crossterm::{event::*, execute, terminal::*};
use tui::{backend::CrosstermBackend, Terminal};

pub struct TableExitStatus {
    pub go_request: Option<String>,
    pub last_requested_action: Option<char>,
    pub rm_selection: Vec<String>,
    pub success: bool,
}

impl StatefulCmdsTable<'_> {
    fn add_handler(&mut self, _exit_status: &mut TableExitStatus) {
        todo!()
    }

    fn go_handler(&self, exit_status: &mut TableExitStatus) {
        if let Some(selected_index) = self.state.selected() {
            let key = self.cmds.borrow()[selected_index].0.clone();
            exit_status.go_request.replace(key);
        }
    }

    fn rm_handler(&mut self, exit_status: &mut TableExitStatus) {
        if let Some(selected_index) = self.state.selected() {
            let key = self.cmds.borrow()[selected_index].0.clone();
            exit_status.rm_selection.push(key);
            self.selected_indices.push(selected_index);
        }
    }

    pub fn render(&mut self, opts: TuiOpts) -> Result<TableExitStatus> {
        let TuiOpts {
            selected_style,
            input_handler,
            ..
        } = &opts;

        let exit_requested = &mut false;
        let mut exit_status = TableExitStatus {
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
        let confirm_dialog_open = &mut false;
        let last_requested_popup: &mut Option<&PopupOpts> = &mut None;
        let _rm_popup = PopupOpts {
            title: "Confirm Deletion",
            message: "Remove {{ ctx }}",
            requires_context: true,
        };

        loop {
            terminal.draw(|frame| {
                let layout = full_win_layout(frame);
                let _table = table_layout(self, frame, layout, *selected_style);

                if let Some(popup_opts) = last_requested_popup {
                    if *confirm_dialog_open {
                        if let Some(selected_index) = self.state.selected() {
                            render_popup(
                                frame,
                                popup_opts,
                                self.cmds.borrow()[selected_index].0.as_ref(),
                            );
                        }
                    }
                }
            })?;

            let event = rx.recv()?;
            if let Event::Mouse(MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column,
                row,
                ..
            }) = event
            {
                terminal.set_cursor(column, row)?;
                self.select(row as usize - 2);
            }

            match event {
                add if input_handler.trigger_go(&add) => {
                    exit_status_ref
                        .last_requested_action
                        .replace(TuiInputHandler::ADD);
                    self.add_handler(exit_status_ref);
                }
                go if input_handler.trigger_go(&go) => {
                    *exit_requested = true;
                    exit_status_ref
                        .last_requested_action
                        .replace(TuiInputHandler::GO);
                    self.go_handler(exit_status_ref);
                    break;
                }
                rm if input_handler.trigger_rm(&rm) => {
                    exit_status_ref
                        .last_requested_action
                        .replace(TuiInputHandler::ADD);
                    self.rm_handler(exit_status_ref)
                }
                a if input_handler.accepts(&a) => {
                    if *confirm_dialog_open {
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
                        *confirm_dialog_open = false;
                    }
                }
                r if input_handler.rejects(&r) => *confirm_dialog_open = false,
                s if input_handler.selects(&s) => {
                    if let Some(popup_opts) = last_requested_popup {
                        if popup_opts.requires_context {
                            *confirm_dialog_open = true;
                        }
                    } else {
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
                }
                u if input_handler.unselects(&u) => {
                    if let Some(selected_index) = self.state.selected() {
                        self.selected_indices.retain(|s| *s != selected_index);
                    }
                }
                e if input_handler.is_exit_trigger(&e) => {
                    disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;
                    break;
                }
                Event::Key(KeyEvent { code, modifiers }) => {
                    if modifiers == KeyModifiers::NONE {
                        match code {
                            KeyCode::Down => self.next(),
                            KeyCode::Up => self.previous(),
                            _ => {}
                        }
                    }
                }
                Event::Mouse(MouseEvent { kind, .. }) => match kind {
                    MouseEventKind::ScrollDown => self.next(),
                    MouseEventKind::ScrollUp => self.previous(),
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(exit_status)
    }
}
