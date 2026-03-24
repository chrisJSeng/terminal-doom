use crate::constants::{MAP_BOUNDS_MIN_SPAN, PLAYER_DEFAULT_ANGLE, PLAYER_DEFAULT_HEALTH};
#[cfg(test)]
use crate::types::CollidesWithMapParams;
use crate::types::{GameState, GameStateInit, MapBounds, MapData, Player};

#[cfg(test)]
use crate::constants::{
    PICKUP_HEALTH_AMOUNT, PICKUP_HEALTH_RADIUS, PLAYER_COLLISION_RADIUS, PLAYER_MAX_HEALTH,
    THING_TYPE_MEDIKIT, THING_TYPE_SOULSPHERE, THING_TYPE_STIMPACK, WAD_INVALID_SIDEDEF_INDEX,
};
#[cfg(test)]
use crate::types::{LineDef, SegmentDistanceParams};

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            angle: PLAYER_DEFAULT_ANGLE,
            health: PLAYER_DEFAULT_HEALTH,
        }
    }

    #[cfg(test)]
    pub fn move_forward(&mut self, distance: f32) {
        self.x += self.angle.cos() * distance;
        self.y += self.angle.sin() * distance;
    }

    #[cfg(test)]
    pub fn move_lateral(&mut self, distance: f32) {
        let strafe_angle = self.angle + std::f32::consts::FRAC_PI_2;
        self.x += strafe_angle.cos() * distance;
        self.y += strafe_angle.sin() * distance;
    }

    #[cfg(test)]
    pub fn rotate(&mut self, angle_delta: f32) {
        self.angle += angle_delta;
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}

impl GameState {
    pub fn new(params: GameStateInit) -> Self {
        let GameStateInit { map, player } = params;

        GameState { player, map }
    }
}

impl MapData {
    pub fn bounds(&self) -> MapBounds {
        let min_x = self
            .vertexes
            .iter()
            .map(|vertex| vertex.x)
            .min()
            .unwrap_or(0) as f32;

        let max_x = self
            .vertexes
            .iter()
            .map(|vertex| vertex.x)
            .max()
            .unwrap_or(1) as f32;

        let min_y = self
            .vertexes
            .iter()
            .map(|vertex| vertex.y)
            .min()
            .unwrap_or(0) as f32;

        let max_y = self
            .vertexes
            .iter()
            .map(|vertex| vertex.y)
            .max()
            .unwrap_or(1) as f32;

        let width = (max_x - min_x).max(MAP_BOUNDS_MIN_SPAN);
        let height = (max_y - min_y).max(MAP_BOUNDS_MIN_SPAN);

        MapBounds {
            min_x,
            min_y,
            width,
            height,
        }
    }
}

#[cfg(test)]
pub fn collides_with_map(params: CollidesWithMapParams<'_>) -> bool {
    let CollidesWithMapParams { player, map } = params;
    let bounds = map.bounds();

    let inside_x_min = player.x >= bounds.min_x + PLAYER_COLLISION_RADIUS;
    let inside_x_max = player.x <= (bounds.min_x + bounds.width) - PLAYER_COLLISION_RADIUS;
    let inside_y_min = player.y >= bounds.min_y + PLAYER_COLLISION_RADIUS;
    let inside_y_max = player.y <= (bounds.min_y + bounds.height) - PLAYER_COLLISION_RADIUS;
    let inside_map_bounds = inside_x_min && inside_x_max && inside_y_min && inside_y_max;

    if !inside_map_bounds {
        return true;
    }

    map.lines
        .iter()
        .filter(|line| is_collision_line(line))
        .any(|line| {
            let (start, end) = match (
                map.vertexes.get(line.start_vertex as usize),
                map.vertexes.get(line.end_vertex as usize),
            ) {
                (Some(start), Some(end)) => (start, end),
                _ => return false,
            };

            let wall_distance = distance_point_to_segment(SegmentDistanceParams {
                px: player.x,
                py: player.y,
                ax: start.x as f32,
                ay: start.y as f32,
                bx: end.x as f32,
                by: end.y as f32,
            });

            wall_distance < PLAYER_COLLISION_RADIUS
        })
}

#[cfg(test)]
pub fn update_game_state(game_state: &mut GameState, _delta_time: f32) -> bool {
    collect_health_pickups(game_state)
}

#[cfg(test)]
fn collect_health_pickups(game_state: &mut GameState) -> bool {
    let player_x = game_state.player.x;
    let player_y = game_state.player.y;
    let pickup_radius_sq = PICKUP_HEALTH_RADIUS * PICKUP_HEALTH_RADIUS;
    let mut consumed_any_pickup = false;

    game_state.map.things.retain(|thing| {
        let is_health_pickup = is_health_pickup_thing_type(thing.thing_type);

        if !is_health_pickup {
            return true;
        }

        let delta_x = thing.x as f32 - player_x;
        let delta_y = thing.y as f32 - player_y;
        let distance_sq = delta_x * delta_x + delta_y * delta_y;
        let is_in_pickup_range = distance_sq <= pickup_radius_sq;

        if !is_in_pickup_range {
            return true;
        }

        consumed_any_pickup = true;
        false
    });

    if consumed_any_pickup {
        game_state.player.health = game_state
            .player
            .health
            .saturating_add(PICKUP_HEALTH_AMOUNT)
            .min(PLAYER_MAX_HEALTH);
    }

    consumed_any_pickup
}

#[cfg(test)]
fn is_health_pickup_thing_type(thing_type: u16) -> bool {
    matches!(
        thing_type,
        THING_TYPE_STIMPACK | THING_TYPE_MEDIKIT | THING_TYPE_SOULSPHERE
    )
}

#[cfg(test)]
fn is_collision_line(line: &LineDef) -> bool {
    line.right_sidedef != WAD_INVALID_SIDEDEF_INDEX
        && line.left_sidedef == WAD_INVALID_SIDEDEF_INDEX
}

#[cfg(test)]
fn distance_point_to_segment(params: SegmentDistanceParams) -> f32 {
    let SegmentDistanceParams {
        px,
        py,
        ax,
        ay,
        bx,
        by,
    } = params;

    let wall_dir_x = bx - ax;
    let wall_dir_y = by - ay;
    let to_player_x = px - ax;
    let to_player_y = py - ay;
    let segment_length_sq = wall_dir_x * wall_dir_x + wall_dir_y * wall_dir_y;

    if segment_length_sq <= f32::EPSILON {
        let start_to_player_x = px - ax;
        let start_to_player_y = py - ay;

        return (start_to_player_x * start_to_player_x + start_to_player_y * start_to_player_y)
            .sqrt();
    }

    let projection = (to_player_x * wall_dir_x + to_player_y * wall_dir_y) / segment_length_sq;
    let clamped_projection = projection.clamp(0.0, 1.0);

    let nearest_x = ax + clamped_projection * wall_dir_x;
    let nearest_y = ay + clamped_projection * wall_dir_y;

    let nearest_to_player_x = px - nearest_x;
    let nearest_to_player_y = py - nearest_y;
    (nearest_to_player_x * nearest_to_player_x + nearest_to_player_y * nearest_to_player_y).sqrt()
}

#[cfg(test)]
#[path = "tests/domain_tests.rs"]
mod tests;
