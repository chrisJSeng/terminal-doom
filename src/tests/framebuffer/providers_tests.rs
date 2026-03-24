use super::*;

#[test]
fn engine_player_state_spawn_default_uses_expected_initial_values() {
    let player_state = EnginePlayerState::spawn_default();

    assert_eq!(player_state.x, 0.0);
    assert_eq!(player_state.y, 0.0);
    assert_eq!(player_state.angle, 0.0);
    assert_eq!(player_state.health, 100);
    assert_eq!(player_state.armor, 0);
    assert_eq!(player_state.bullets, 0);
    assert_eq!(player_state.shells, 0);
}
