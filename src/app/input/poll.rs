use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyEventKind};

use crate::constants::INPUT_POLL_MILLIS;
use crate::types::{InputAction, ProcessKeyEventParams};

use super::mapping::map_key_to_action;
use super::state::is_held_action;

pub fn poll_input_actions() -> io::Result<(Vec<InputAction>, Vec<InputAction>)> {
    if !event::poll(Duration::from_millis(INPUT_POLL_MILLIS))? {
        return Ok((Vec::new(), Vec::new()));
    }

    let mut pressed = Vec::new();
    let mut released = Vec::new();

    loop {
        let event = event::read()?;

        if let Event::Key(key) = event {
            process_key_event(ProcessKeyEventParams {
                key,
                pressed: &mut pressed,
                released: &mut released,
            });
        }

        if !event::poll(Duration::ZERO)? {
            break;
        }
    }

    Ok((pressed, released))
}

fn process_key_event(params: ProcessKeyEventParams) {
    let ProcessKeyEventParams {
        key,
        pressed,
        released,
    } = params;

    match key.kind {
        KeyEventKind::Press => {
            if let Some(action) = map_key_to_action(key) {
                pressed.push(action);
            }
        }
        KeyEventKind::Repeat => {
            if let Some(action) = map_key_to_action(key).filter(|&a| is_held_action(a)) {
                pressed.push(action);
            }
        }
        KeyEventKind::Release => {
            if let Some(action) = map_key_to_action(key) {
                released.push(action);
            }
        }
    }
}
