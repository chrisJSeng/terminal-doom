use super::*;

#[test]
fn expire_stale_removes_old_held_actions() {
    let mut state = InputState::new();
    let t0 = Instant::now();
    state.update(InputStateUpdateParams {
        pressed: &[InputAction::MoveForward],
        now: t0,
    });
    assert_eq!(state.ordered_held_actions(), vec![InputAction::MoveForward]);

    let t1 = t0 + HELD_ACTION_STALE_TIMEOUT + Duration::from_millis(1);
    state.update(InputStateUpdateParams {
        pressed: &[],
        now: t1,
    });
    assert!(state.ordered_held_actions().is_empty());
}

#[test]
fn discrete_pressed_actions_deduplicates_and_excludes_held() {
    let mut state = InputState::new();
    let t0 = Instant::now();
    state.update(InputStateUpdateParams {
        pressed: &[InputAction::MoveForward],
        now: t0,
    });

    let pressed = vec![
        InputAction::MoveForward,
        InputAction::WeaponPrev,
        InputAction::WeaponPrev,
    ];
    let discrete = state.discrete_pressed_actions(&pressed);
    assert_eq!(discrete, vec![InputAction::WeaponPrev]);
}

#[test]
fn opposing_held_actions_cancel_each_other() {
    let mut state = InputState::new();
    let t0 = Instant::now();
    state.update(InputStateUpdateParams {
        pressed: &[InputAction::MoveForward, InputAction::MoveBackward],
        now: t0,
    });
    let held = state.ordered_held_actions();
    assert!(!held.contains(&InputAction::MoveForward));
    assert!(!held.contains(&InputAction::MoveBackward));
}
