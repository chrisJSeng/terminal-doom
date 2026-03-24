mod mapping;
mod poll;
mod state;

pub use crate::types::InputState;
pub use poll::poll_input_actions;

#[cfg(test)]
pub(crate) use mapping::map_key_to_action;

#[cfg(test)]
#[path = "../../tests/app/input_tests.rs"]
mod tests;
