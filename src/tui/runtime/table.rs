use crate::{
    prelude::*,
    tui::{
        layouts::{full_win_layout, table_layout},
        prelude::*,
        widgets::*,
    },
};
use crossterm::{event::*, execute, terminal::*};
use tui::{backend::CrosstermBackend, Terminal};

#[derive(Default)]
pub struct ScanDirOpts {
    pub depth: String,
    pub ext_filters: String,
    pub file_type_filter: String,
    pub raw_filters: String,
    pub regex_filters: String,
}

#[derive(Default)]
pub struct AddCmdUi {
    pub key: String,
    pub bin: String,
    pub joined_args: String,
    pub joined_aliases: String,
    pub encoder: String,
    pub permissions: String,
    pub query_which: String,
    pub scan_dir: Option<ScanDirOpts>,
}

fn parse_with_delim(arg: String, delimiter: &str) -> Option<Vec<String>> {
    let split: Vec<String> = arg
        .split(delimiter)
        .filter_map(|a| {
            if a.is_empty() {
                None
            } else {
                Some(a.to_owned())
            }
        })
        .collect();

    if split.is_empty() {
        None
    } else {
        Some(split)
    }
}

impl AddCmdUi {
    pub fn to_cmd(self) -> Result<(String, GeneratedCommand)> {
        use crate::cli::types::{
            aliases_from_arg, args_from_arg, encoder_from_arg, exts_filter_from_arg,
            file_type_filter_from_arg, permissions_from_arg, recursion_limit_from_arg,
        };

        let aliases = aliases_from_arg(&self.joined_aliases).ok();
        let dfl_args = args_from_arg(&self.joined_args).ok();
        let encoder = encoder_from_arg(&self.encoder).ok();
        let permissions = permissions_from_arg(&self.permissions).or_else(|e| bail!(e))?;
        let query_which = match self.query_which.as_str() {
            "y" | "yes" | "true" => true,
            "f" | "n" | "no" | "false" => false,
            _ => return Err(anyhow!("not a valid input!")),
        };

        let (scan_dir, filter) = match self.scan_dir {
            None => (ScanDirKind::None, FiltersKind::None),
            Some(ScanDirOpts {
                depth,
                ext_filters,
                file_type_filter,
                raw_filters,
                regex_filters,
            }) => {
                let scan_dir = recursion_limit_from_arg(&depth).or_else(|e| bail!(e))?;
                let mut filters = vec![];

                if let Ok(ext_filters) = exts_filter_from_arg(&ext_filters) {
                    filters.push(ext_filters);
                }
                if let Ok(file_type_filter) = file_type_filter_from_arg(&file_type_filter) {
                    filters.push(file_type_filter);
                }
                if let Some(ref mut regex_filters) =
                    parse_with_delim(regex_filters, " ;;; ").map(|mut f| {
                        f.drain(..)
                            .map(|f| FilterKind::RegEx(f))
                            .collect::<Vec<FilterKind>>()
                    })
                {
                    filters.append(regex_filters);
                }
                if let Some(ref mut raw_filters) =
                    parse_with_delim(raw_filters, " ;;; ").map(|mut f| {
                        f.drain(..)
                            .map(|f| FilterKind::Raw(f))
                            .collect::<Vec<FilterKind>>()
                    })
                {
                    filters.append(raw_filters);
                }

                match filters.len() {
                    0 => (scan_dir, FiltersKind::None),
                    1 => (
                        scan_dir,
                        FiltersKind::One(filters.drain(..).nth(0).unwrap()),
                    ),
                    _ => (scan_dir, FiltersKind::Many(filters)),
                }
            }
        };

        Ok((
            self.key,
            GeneratedCommand {
                aliases,
                bin: self.bin,
                dfl_args,
                encoder,
                filter,
                permissions,
                query_which,
                scan_dir,
            },
        ))
    }
}

pub struct TableExitStatus {
    pub go_request: Option<String>,
    pub last_requested_action: Option<char>,
    pub new_cmd: Option<AddCmdUi>,
    pub rm_selection: Vec<String>,
    pub success: bool,
}

use std::{cell::RefCell, sync::Mutex};

impl StatefulCmdsTable<'_> {
    fn _edit_handler(&mut self, _exit_status: &mut TableExitStatus) {
        todo!();
    }

    fn go_handler(&self, exit_status: &mut TableExitStatus) {
        if let Some(selected_index) = self.state.selected() {
            let key = self.cmds.borrow()[selected_index].0.to_string();
            exit_status.go_request.replace(key);
        }
    }

    fn rm_handler(&mut self, exit_status: &mut TableExitStatus) {
        if let Some(selected_index) = self.state.selected() {
            let key = self.cmds.borrow()[selected_index].0.to_string();
            exit_status.rm_selection.push(key);
            self.selected_indices.push(selected_index);
        }
    }

    pub fn render(&mut self, opts: TuiOpts) -> Result<TableExitStatus> {
        let TuiOpts {
            selected_style,
            input_handler,
        } = &opts;

        let exit_requested = &mut false;
        let mut exit_status = TableExitStatus {
            new_cmd: None,
            go_request: None,
            last_requested_action: None,
            rm_selection: vec![],
            success: true,
        };
        let exit_status_ref = &mut exit_status;

        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let rx = spawn_event_loop();

        let last_requested_popup: RefCell<Mutex<PopupWidget>> =
            RefCell::from(Mutex::from(PopupWidget::default()));

        loop {
            terminal.draw(|frame| {
                let layout = full_win_layout(frame);
                let _table = table_layout(self, frame, layout, *selected_style);

                if let Ok(popup_guard) = last_requested_popup.borrow_mut().lock() {
                    (*popup_guard).render(frame);
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

            if let Ok(mut popup_guard) = last_requested_popup.borrow_mut().lock() {
                let mut request_close = false;
                if let PopupWidget::Add(ref mut seq) = *popup_guard {
                    match event {
                        Event::Key(KeyEvent { code, modifiers }) => {
                            if let KeyCode::Char(c) = code {
                                if modifiers == KeyModifiers::CONTROL && c == 'c' {
                                    request_close = true;
                                } else {
                                    seq.print(c);
                                }
                            }

                            if let KeyCode::Enter = code {
                                seq.push();
                            }
                            if let KeyCode::Backspace = code {
                                seq.delete();
                            }
                            if let KeyCode::Left = code {
                                seq.pop();
                            }
                            if let KeyCode::Esc = code {
                                request_close = true;
                            }
                        }
                        _ => {}
                    }

                    if seq.done() {
                        seq.populate(&mut exit_status_ref.new_cmd);
                        request_close = true;
                    }

                    if !request_close {
                        continue;
                    }
                }

                if request_close {
                    *popup_guard = PopupWidget::closed();
                    continue;
                }
            }

            match event {
                add if input_handler.trigger_add(&add) => {
                    if let Ok(mut guard) = last_requested_popup.borrow_mut().lock() {
                        *guard = PopupWidget::Add(AddSequence::new());
                    }
                    exit_status_ref
                        .last_requested_action
                        .replace(TuiInputHandler::ADD);
                }
                go if input_handler.trigger_go(&go) => {
                    *exit_requested = true;
                    exit_status_ref
                        .last_requested_action
                        .replace(TuiInputHandler::GO);
                    self.go_handler(exit_status_ref);
                }
                rm if input_handler.trigger_rm(&rm) => {
                    if let Ok(mut guard) = last_requested_popup.borrow_mut().lock() {
                        if let Some(selected_index) = self.state.selected() {
                            *guard = PopupWidget::Confirm {
                                title: "Confirm Deletion",
                                message: "Remove {{ ctx }}",
                                context: Some(self.cmds.borrow()[selected_index].0.as_ref()),
                            };
                        }
                    }
                    exit_status_ref
                        .last_requested_action
                        .replace(TuiInputHandler::RM);
                    self.rm_handler(exit_status_ref)
                }
                a if input_handler.accepts(&a) => {
                    exit_status_ref.success = true;
                    if let Ok(mut popup) = last_requested_popup.borrow_mut().lock() {
                        if (*popup).is_open() {
                            *popup = PopupWidget::closed();
                        }
                    }
                }
                r if input_handler.rejects(&r) => {
                    exit_status_ref.success = false;
                    if let Ok(mut popup) = last_requested_popup.borrow_mut().lock() {
                        *popup = PopupWidget::closed();
                    }
                }
                // s if input_handler.selects(&s) => {}
                u if input_handler.unselects(&u) => {
                    if let Some(selected_index) = self.state.selected() {
                        self.selected_indices.retain(|s| *s != selected_index);
                    }
                }
                e if input_handler.is_exit_trigger(&e) => *exit_requested = true,
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
