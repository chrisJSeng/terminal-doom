use super::*;
use crate::constants::{THING_TYPE_BARREL, THING_TYPE_STIMPACK};
use crate::domain::CollidesWithMapParams;
use crate::tests::fixtures;
use crate::types::{MapData, Vertex};

struct ApproxEqParams {
    left: f32,
    right: f32,
}

fn approx_eq(params: ApproxEqParams) {
    let ApproxEqParams { left, right } = params;

    assert!((left - right).abs() < 0.0001, "left={left} right={right}");
}

#[test]
fn player_new_initializes_default_pose_and_health() {
    let player = Player::new(12.5, 24.0);

    assert_eq!(player.x, 12.5);
    assert_eq!(player.y, 24.0);
    assert_eq!(player.angle, 0.0);
    assert_eq!(player.health, 100);
}

#[test]
fn player_move_forward_uses_current_angle() {
    let mut player = Player::new(10.0, 20.0);
    player.angle = std::f32::consts::FRAC_PI_2;

    player.move_forward(5.0);

    approx_eq(ApproxEqParams {
        left: player.x,
        right: 10.0,
    });
    approx_eq(ApproxEqParams {
        left: player.y,
        right: 25.0,
    });
}

#[test]
fn player_move_lateral_strafes_perpendicular_to_view() {
    let mut player = Player::new(10.0, 20.0);

    player.move_lateral(5.0);

    approx_eq(ApproxEqParams {
        left: player.x,
        right: 10.0,
    });
    approx_eq(ApproxEqParams {
        left: player.y,
        right: 25.0,
    });
}

#[test]
fn player_rotate_accumulates_angle_delta() {
    let mut player = Player::new(0.0, 0.0);

    player.rotate(1.25);
    player.rotate(-0.5);

    approx_eq(ApproxEqParams {
        left: player.angle,
        right: 0.75,
    });
}

#[test]
fn player_is_alive_depends_on_health() {
    let alive_player = Player {
        x: 0.0,
        y: 0.0,
        angle: 0.0,
        health: 1,
    };
    let dead_player = Player {
        x: 0.0,
        y: 0.0,
        angle: 0.0,
        health: 0,
    };

    assert!(alive_player.is_alive());
    assert!(!dead_player.is_alive());
}

#[test]
fn map_bounds_uses_extreme_vertex_coordinates() {
    let map = MapData {
        vertexes: vec![
            Vertex { x: -32, y: 48 },
            Vertex { x: 96, y: -16 },
            Vertex { x: 48, y: 64 },
        ],
        lines: vec![],
        sidedefs: vec![],
        sectors: vec![],
        palette: vec![],
        textures: vec![],
        flats: vec![],
        things: vec![],
        bsp_line_indices: vec![],
    };

    let bounds = map.bounds();

    assert_eq!(bounds.min_x, -32.0);
    assert_eq!(bounds.min_y, -16.0);
    assert_eq!(bounds.width, 128.0);
    assert_eq!(bounds.height, 80.0);
}

#[test]
fn map_bounds_falls_back_to_minimum_size_when_no_vertexes_exist() {
    let map = MapData {
        vertexes: vec![],
        lines: vec![],
        sidedefs: vec![],
        sectors: vec![],
        palette: vec![],
        textures: vec![],
        flats: vec![],
        things: vec![],
        bsp_line_indices: vec![],
    };

    let bounds = map.bounds();

    assert_eq!(bounds.min_x, 0.0);
    assert_eq!(bounds.min_y, 0.0);
    assert_eq!(bounds.width, 1.0);
    assert_eq!(bounds.height, 1.0);
}

#[test]
fn collides_with_map_ignores_non_solid_linedef() {
    let (map, player) = fixtures::collision::sample_non_solid_collision_case();

    let hit = collides_with_map(CollidesWithMapParams {
        player: &player,
        map: &map,
    });
    assert!(!hit);
}

#[test]
fn update_game_state_collects_health_pickup_once_and_removes_thing() {
    let mut game_state = fixtures::gameplay::sample_pickup_game_state(THING_TYPE_STIMPACK);

    let first_pick = update_game_state(&mut game_state, 0.016);
    let second_pick = update_game_state(&mut game_state, 0.016);

    assert!(first_pick);
    assert!(!second_pick);
    assert_eq!(game_state.player.health, 65);
    assert!(game_state.map.things.is_empty());
}

#[test]
fn update_game_state_ignores_non_pickup_things() {
    let mut game_state = fixtures::gameplay::sample_pickup_game_state(THING_TYPE_BARREL);

    let picked = update_game_state(&mut game_state, 0.016);

    assert!(!picked);
    assert_eq!(game_state.player.health, 50);
    assert_eq!(game_state.map.things.len(), 1);
}
