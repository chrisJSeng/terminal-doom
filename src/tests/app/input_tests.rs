use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

use super::*;
use crate::types::InputAction;

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

fn key_with_ctrl(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

#[test]
fn map_key_to_action_maps_supported_keys() {
    assert!(matches!(
        map_key_to_action(key(KeyCode::Esc)),
        Some(InputAction::Quit)
    ));
    assert!(matches!(
        map_key_to_action(key(KeyCode::Up)),
        Some(InputAction::MoveForward)
    ));
    assert!(matches!(
        map_key_to_action(key(KeyCode::Char('w'))),
        Some(InputAction::MoveForward)
    ));
    assert!(matches!(
        map_key_to_action(key(KeyCode::Char('s'))),
        Some(InputAction::MoveBackward)
    ));
    assert!(matches!(
        map_key_to_action(key(KeyCode::Left)),
        Some(InputAction::RotateLeft)
    ));
    assert!(matches!(
        map_key_to_action(key(KeyCode::Char('a'))),
        Some(InputAction::RotateLeft)
    ));
    assert!(matches!(
        map_key_to_action(key(KeyCode::Char('d'))),
        Some(InputAction::RotateRight)
    ));
    assert!(matches!(
        map_key_to_action(key(KeyCode::Char('z'))),
        Some(InputAction::StrafeLeft)
    ));
    assert!(matches!(
        map_key_to_action(key(KeyCode::Char('c'))),
        Some(InputAction::StrafeRight)
    ));
    assert!(matches!(
        map_key_to_action(key(KeyCode::Char(' '))),
        Some(InputAction::Fire)
    ));
    assert!(matches!(
        map_key_to_action(key_with_ctrl(KeyCode::Char('a'))),
        Some(InputAction::Use)
    ));
    assert!(matches!(
        map_key_to_action(key(KeyCode::Tab)),
        Some(InputAction::ToggleAutomap)
    ));
    assert!(matches!(
        map_key_to_action(key(KeyCode::Char('p'))),
        Some(InputAction::OpenMenu)
    ));
    assert!(matches!(
        map_key_to_action(key(KeyCode::Enter)),
        Some(InputAction::ConfirmMenu)
    ));
    assert!(matches!(
        map_key_to_action(key(KeyCode::Backspace)),
        Some(InputAction::BackMenu)
    ));
    assert!(matches!(
        map_key_to_action(key(KeyCode::Char('q'))),
        Some(InputAction::WeaponPrev)
    ));
    assert!(matches!(
        map_key_to_action(key(KeyCode::Char('e'))),
        Some(InputAction::WeaponNext)
    ));
}

#[test]
fn map_key_to_action_returns_none_for_unknown_key() {
    assert!(map_key_to_action(key(KeyCode::Char('v'))).is_none());
}
