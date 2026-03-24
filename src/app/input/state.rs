use std::collections::HashSet;
use std::time::{Duration, Instant};

use crate::constants::INPUT_HELD_ACTION_STALE_TIMEOUT_MILLIS;
use crate::types::{InputAction, InputState, InputStateUpdateParams, ResolvedOpposedActionParams};

const HELD_ACTION_STALE_TIMEOUT: Duration =
    Duration::from_millis(INPUT_HELD_ACTION_STALE_TIMEOUT_MILLIS);

impl InputState {
    pub fn new() -> Self {
        InputState {
            held_actions: Default::default(),
        }
    }

    pub fn update(&mut self, params: InputStateUpdateParams) {
        let InputStateUpdateParams { pressed, now } = params;

        for &action in pressed {
            if is_held_action(action) {
                self.held_actions.insert(action, now);
            }
        }

        self.expire_stale(now);
    }

    pub fn ordered_held_actions(&self) -> Vec<InputAction> {
        let mut result = Vec::new();

        let forward_pair = (
            self.held_actions.contains_key(&InputAction::MoveForward),
            self.held_actions.contains_key(&InputAction::MoveBackward),
        );

        let forward_params = ResolvedOpposedActionParams {
            pair: forward_pair,
            first: InputAction::MoveForward,
            second: InputAction::MoveBackward,
        };

        if let Some(action) = resolved_opposed_action(forward_params) {
            result.push(action);
        }

        let strafe_pair = (
            self.held_actions.contains_key(&InputAction::StrafeLeft),
            self.held_actions.contains_key(&InputAction::StrafeRight),
        );

        let strafe_params = ResolvedOpposedActionParams {
            pair: strafe_pair,
            first: InputAction::StrafeLeft,
            second: InputAction::StrafeRight,
        };

        if let Some(action) = resolved_opposed_action(strafe_params) {
            result.push(action);
        }

        let rotate_pair = (
            self.held_actions.contains_key(&InputAction::RotateLeft),
            self.held_actions.contains_key(&InputAction::RotateRight),
        );

        let rotate_params = ResolvedOpposedActionParams {
            pair: rotate_pair,
            first: InputAction::RotateLeft,
            second: InputAction::RotateRight,
        };

        if let Some(action) = resolved_opposed_action(rotate_params) {
            result.push(action);
        }

        if self.held_actions.contains_key(&InputAction::Fire) {
            result.push(InputAction::Fire);
        }

        result
    }

    pub fn discrete_pressed_actions(&self, pressed: &[InputAction]) -> Vec<InputAction> {
        let mut seen = HashSet::new();

        pressed
            .iter()
            .filter(|&&action| !self.held_actions.contains_key(&action))
            .filter(|&&action| seen.insert(action))
            .copied()
            .collect()
    }

    fn expire_stale(&mut self, now: Instant) {
        self.held_actions
            .retain(|_, instant| now.duration_since(*instant) < HELD_ACTION_STALE_TIMEOUT);
    }
}

fn resolved_opposed_action(params: ResolvedOpposedActionParams) -> Option<InputAction> {
    let ResolvedOpposedActionParams {
        pair,
        first,
        second,
    } = params;

    match pair {
        (true, false) => Some(first),
        (false, true) => Some(second),
        _ => None,
    }
}

pub fn is_held_action(action: InputAction) -> bool {
    matches!(
        action,
        InputAction::MoveForward
            | InputAction::MoveBackward
            | InputAction::StrafeLeft
            | InputAction::StrafeRight
            | InputAction::RotateLeft
            | InputAction::RotateRight
            | InputAction::Fire
    )
}

#[cfg(test)]
#[path = "../../tests/app/input_state_tests.rs"]
mod tests;
