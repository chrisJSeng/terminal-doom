use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::types::InputAction;

pub(crate) fn map_key_to_action(key: KeyEvent) -> Option<InputAction> {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        return Some(InputAction::Use);
    }

    match key.code {
        KeyCode::Esc => Some(InputAction::Quit),
        KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => Some(InputAction::MoveForward),
        KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => Some(InputAction::MoveBackward),
        KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => Some(InputAction::RotateLeft),
        KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => Some(InputAction::RotateRight),
        KeyCode::Char('z') | KeyCode::Char('Z') => Some(InputAction::StrafeLeft),
        KeyCode::Char('c') | KeyCode::Char('C') => Some(InputAction::StrafeRight),
        KeyCode::Char(' ') => Some(InputAction::Fire),
        KeyCode::Tab => Some(InputAction::ToggleAutomap),
        KeyCode::Char('p') | KeyCode::Char('P') => Some(InputAction::OpenMenu),
        KeyCode::Enter => Some(InputAction::ConfirmMenu),
        KeyCode::Backspace => Some(InputAction::BackMenu),
        KeyCode::Char('q') | KeyCode::Char('Q') => Some(InputAction::WeaponPrev),
        KeyCode::Char('e') | KeyCode::Char('E') => Some(InputAction::WeaponNext),
        _ => None,
    }
}
