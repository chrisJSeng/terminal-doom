use crate::domain::collides_with_map;
use crate::types::{CollidesWithMapParams, TryMovePlayerWithCollisionParams};

pub fn try_move_player_with_collision(params: TryMovePlayerWithCollisionParams<'_>) -> bool {
    let TryMovePlayerWithCollisionParams {
        game_state,
        distance,
    } = params;

    let previous_player = game_state.player;

    game_state.player.move_forward(distance);

    let hit_wall = collides_with_map(CollidesWithMapParams {
        player: &game_state.player,
        map: &game_state.map,
    });

    if hit_wall {
        game_state.player = previous_player;
    }

    hit_wall
}
