mod game_loop;
#[cfg(test)]
mod movement;

#[cfg(test)]
pub use crate::types::TryMovePlayerWithCollisionParams;
pub use game_loop::run_game_loop;
#[cfg(test)]
pub use movement::try_move_player_with_collision;

#[cfg(test)]
#[path = "../../tests/app/runtime_tests.rs"]
mod tests;
