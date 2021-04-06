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

use parking_lot::Mutex;

impl StatefulCmdsTable<'_> {
    fn _edit_handler(&mut self, _exit_status: &mut TableExitStatus) {
        todo!();
    }

    fn go_handler(&self, exit_status: &mut TableExitStatus, selected_index: usize) {
        let key = self.cmds.borrow()[selected_index].0.to_string();
        exit_status.last_requested_action.replace(EventHandler::GO);
        exit_status.go_request.replace(key);
        exit_status.success = true;
    }

    fn rm_handler(&mut self, exit_status: &mut TableExitStatus, selected_index: usize) {
        let key = self.cmds.borrow()[selected_index].0.to_string();
        exit_status.last_requested_action.replace(EventHandler::RM);
        exit_status.rm_selection.push(key);
        self.selected_indices.push(selected_index);
        exit_status.success = true;
    }

    pub fn render(&mut self, mut event_handler: StatefulEventHandler) -> Result<TableExitStatus> {
        // app state
        let exit_requested = &mut false;
        let request_popup_close = &mut false;
        let popup_state = &mut PopupState::Closed;
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
                    let popup_state = (*ui_guard.lock()).state;
                    popup_state.render(frame);
                }
            })?;

            let event = rx.recv()?;
            if let Some((column, row)) = Event::get_coords(&event) {
                terminal.set_cursor(column, row)?;
                self.select(row as usize - 2);
            }

            {
                let ui_guard = ui_state.borrow_mut();
                let StatefulEventHandler {
                    ref mut state,
                    handler,
                } = *ui_guard.lock();

                match state {
                    PopupState::Add(ref mut seq) => {
                        use crossterm::event::{
                            Event as CrossEvent, KeyCode, KeyEvent, KeyModifiers,
                        };
                        match event {
                            a if handler.accepts(&a) => {
                                seq.push();
                            }
                            r if handler.rejects(&r) => {
                                *request_popup_close = true;
                            }
                            CrossEvent::Key(KeyEvent {
                                code: KeyCode::Left,
                                modifiers: KeyModifiers::NONE,
                            }) => {
                                seq.pop();
                            }
                            CrossEvent::Key(KeyEvent {
                                code: KeyCode::Backspace,
                                modifiers: KeyModifiers::NONE,
                            }) => {
                                seq.delete();
                            }
                            CrossEvent::Key(KeyEvent {
                                code: KeyCode::Char('c'),
                                modifiers: KeyModifiers::CONTROL,
                            }) => {
                                *request_popup_close = true;
                            }
                            CrossEvent::Key(KeyEvent {
                                code: KeyCode::Char(c),
                                modifiers: KeyModifiers::NONE,
                            }) => {
                                seq.print(c);
                            }
                            _ => {}
                        }

                        if seq.done() {
                            seq.populate(&mut exit_status_ref.new_cmd);
                            exit_status_ref.success = true;
                            *request_popup_close = true;
                        }

                        if !*request_popup_close {
                            continue;
                        }
                    }
                    PopupState::Edit => {}
                    PopupState::RmConfirm => {}
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
                                *state = PopupState::RmConfirm;
                                // *guard.lock() = PopupState::Confirm {
                                // title: "Confirm Deletion",
                                // message: "Remove {{ ctx }}",
                                // context: Some(self.cmds.borrow()[selected_index].0.as_ref()),
                                // };

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

                if *request_popup_close {
                    *state = PopupState::Closed;
                    *request_popup_close = false;
                    continue;
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
