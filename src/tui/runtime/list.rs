use crate::{
    prelude::*,
    tui::{
        events::Event,
        layouts::list::main_layout,
        opts::{TuiCallback, TuiOpts},
        widgets::{
            list::{ListEntry, StatefulList},
            popup::render_popup,
        },
    },
};
use termion::event::{Key, MouseButton, MouseEvent};

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
        events,
        mut callback,
        popup_options,
        selected_style,
        mut terminal,
        input_handler,
    }: TuiOpts<'opts, F>,
    list_items: &'items std::cell::RefCell<&'items mut Vec<T>>,
) -> Result<char> {
    let mut app = StatefulList::new(list_items);
    let mut popup_visible = false;
    let mut last_pressed_char = 'q';
    let list_selections: &mut Vec<usize> = &mut vec![];

    loop {
        terminal.draw(|frame| {
            let layout = main_layout(frame);
            let items_list = match list_selections.is_empty() {
                false => {
                    app.styled_tui_list(selected_style.sudoku("style missing :("), list_selections)
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

        let last_input = events.next()?;
        if let Event::KeyPress(Key::Char(k)) = last_input {
            last_pressed_char = k;
        } else if let Event::Mouse(MouseEvent::Press(MouseButton::Left, x, y)) = last_input {
            terminal.set_cursor(x, y)?;
            app.select(y as usize - 2);
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
            e if input_handler.is_exit_trigger(&e) => break,
            Event::KeyPress(key) => match key {
                Key::Down => app.next(),
                Key::Up => app.previous(),
                _ => {}
            },
            Event::Mouse(MouseEvent::Press(MouseButton::WheelDown, ..)) => app.next(),
            Event::Mouse(MouseEvent::Press(MouseButton::WheelUp, ..)) => app.previous(),
            _ => {}
        }
    }

    Ok(last_pressed_char)
}
