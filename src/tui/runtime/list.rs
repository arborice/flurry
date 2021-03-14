use crate::{
    prelude::*,
    tui::{
        layouts::list::main_layout,
        opts::{TuiCallback, TuiOpts},
        widgets::{
            list::{ListEntry, StatefulList},
            popup::render_popup,
        },
    },
};
use crossterm::{
    event::{
        DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
        MouseButton, MouseEvent, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::sync::mpsc;
use tui::{backend::CrosstermBackend, Terminal};

fn callback_handler<F: FnMut(usize), T: ListEntry>(
    stateful_list: &StatefulList<T>,
    callback: &mut TuiCallback<F>,
) -> (bool, Option<usize>) {
    if let Some(selected_index) = stateful_list.state.selected() {
        match callback {
            TuiCallback::Halting(ref mut halting) => {
                halting(selected_index);
                (true, Some(selected_index))
            }
            TuiCallback::NonHalting(ref mut non_halting) => {
                non_halting(selected_index);
                (false, Some(selected_index))
            }
        }
    } else {
        (false, None)
    }
}

pub fn render<'opts, 'items, F: FnMut(usize), T: ListEntry>(
    TuiOpts {
        mut callback,
        popup_options,
        selected_style,
        input_handler,
    }: TuiOpts<'opts, F>,
    list_items: &'items std::cell::RefCell<&'items mut Vec<T>>,
) -> Result<char> {
    let mut app = StatefulList::new(list_items);
    let mut popup_visible = false;
    let mut last_pressed_char = 'q';
    let list_selections: &mut Vec<usize> = &mut vec![];

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (_, rx) = mpsc::channel();

    loop {
        terminal.draw(|frame| {
            let layout = main_layout(frame);
            let items_list = match list_selections.is_empty() {
                false => {
                    app.styled_tui_list(selected_style.seppuku("style missing :("), list_selections)
                }
                true => app.tui_list(),
            };

            frame.render_stateful_widget(items_list, layout[0], &mut app.state);

            if let Some(opts) = popup_options {
                if popup_visible {
                    render_popup(&app, frame, opts);
                }
            }
        })?;

        let last_input = rx.recv()?;
        if let Event::Key(KeyEvent {
            code: KeyCode::Char(k),
            ..
        }) = last_input
        {
            last_pressed_char = k;
        } else if let Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column,
            row,
            ..
        }) = last_input
        {
            terminal.set_cursor(column, row)?;
            app.select(row as usize - 2);
        }

        match last_input {
            a if input_handler.accepts(&a) => {
                if popup_visible {
                    let (halt, selected_index) = callback_handler(&app, &mut callback);
                    if halt {
                        break;
                    }
                    if let Some(index) = selected_index {
                        if selected_style.is_some() {
                            list_selections.push(index);
                        }
                    }
                    popup_visible = false;
                }
            }
            r if input_handler.rejects(&r) => {
                if popup_visible {
                    popup_visible = false;
                }
            }
            s if input_handler.selects(&s) => {
                if popup_options.is_some() && !popup_visible {
                    popup_visible = true;
                } else {
                    let (halt, selected_index) = callback_handler(&app, &mut callback);
                    if halt {
                        break;
                    }
                    if let Some(index) = selected_index {
                        if selected_style.is_some() {
                            list_selections.push(index);
                        }
                    }
                }
            }
            u if input_handler.unselects(&u) => {
                if let Some(selected_index) = app.state.selected() {
                    list_selections.retain(|s| *s != selected_index);
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
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        _ => {}
                    }
                }
            }
            Event::Mouse(MouseEvent { kind, .. }) => match kind {
                MouseEventKind::ScrollDown => app.next(),
                MouseEventKind::ScrollUp => app.previous(),
                _ => {}
            },
            _ => {}
        }
    }

    Ok(last_pressed_char)
}
