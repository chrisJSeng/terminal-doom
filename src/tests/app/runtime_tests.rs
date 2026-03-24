use super::*;
use crate::tests::fixtures;

#[test]
fn try_move_player_with_collision_moves_when_no_collision() {
    let mut game_state = fixtures::gameplay::sample_game_state();
    let original_x = game_state.player.x;

    let collided = try_move_player_with_collision(TryMovePlayerWithCollisionParams {
        game_state: &mut game_state,
        distance: 1.0,
    });

    assert!(!collided);
    assert!(game_state.player.x > original_x);
}

#[test]
fn try_move_player_with_collision_rolls_back_on_collision() {
    let mut game_state = fixtures::gameplay::sample_game_state();
    let original_x = game_state.player.x;
    let original_y = game_state.player.y;

    let collided = try_move_player_with_collision(TryMovePlayerWithCollisionParams {
        game_state: &mut game_state,
        distance: 200.0,
    });

    assert!(collided);
    assert_eq!(game_state.player.x, original_x);
    assert_eq!(game_state.player.y, original_y);
}
