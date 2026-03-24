use crate::constants::{
    FRAMEBUFFER_DEFAULT_LIGHT_LEVEL, FRAMEBUFFER_MAX_CHANNEL_VALUE_U8, FRAMEBUFFER_RGB_CHANNELS,
    FRAMEBUFFER_TEST_HEIGHT, FRAMEBUFFER_TEST_WIDTH, WAD_INVALID_SIDEDEF_INDEX,
};
use crate::types::{
    CFrameProvider, EngineInitParams, EnginePlayerState, EngineSetPlayerSpawnParams,
    EngineWorldUploadParams, InputAction, MapData, Player, RgbFrame,
};

use super::c_backend::{
    engine_fill_frame, engine_get_state, engine_init, engine_set_input, engine_set_player_spawn,
    engine_set_world, engine_step,
};
use super::FrameProvider;
use super::{
    ENGINE_INPUT_BACKWARD, ENGINE_INPUT_BACK_MENU, ENGINE_INPUT_CONFIRM_MENU, ENGINE_INPUT_FIRE,
    ENGINE_INPUT_FORWARD, ENGINE_INPUT_OPEN_MENU, ENGINE_INPUT_STRAFE_LEFT,
    ENGINE_INPUT_STRAFE_RIGHT, ENGINE_INPUT_TOGGLE_AUTOMAP, ENGINE_INPUT_TURN_LEFT,
    ENGINE_INPUT_TURN_RIGHT, ENGINE_INPUT_USE, ENGINE_INPUT_WEAPON_NEXT, ENGINE_INPUT_WEAPON_PREV,
};

impl EnginePlayerState {
    pub fn spawn_default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            angle: 0.0,
            health: 100,
            armor: 0,
            bullets: 0,
            shells: 0,
        }
    }
}

impl CFrameProvider {
    pub fn new() -> Self {
        Self {
            width: FRAMEBUFFER_TEST_WIDTH,
            height: FRAMEBUFFER_TEST_HEIGHT,
            tick: 0,
            input_mask: 0,
            engine_initialized: false,
            engine_player_state: EnginePlayerState::spawn_default(),
        }
    }

    pub fn queue_input_action(&mut self, action: InputAction) {
        match action {
            InputAction::MoveForward => self.input_mask |= ENGINE_INPUT_FORWARD,
            InputAction::MoveBackward => self.input_mask |= ENGINE_INPUT_BACKWARD,
            InputAction::RotateLeft => self.input_mask |= ENGINE_INPUT_TURN_LEFT,
            InputAction::RotateRight => self.input_mask |= ENGINE_INPUT_TURN_RIGHT,
            InputAction::StrafeLeft => self.input_mask |= ENGINE_INPUT_STRAFE_LEFT,
            InputAction::StrafeRight => self.input_mask |= ENGINE_INPUT_STRAFE_RIGHT,
            InputAction::Fire => self.input_mask |= ENGINE_INPUT_FIRE,
            InputAction::Use => self.input_mask |= ENGINE_INPUT_USE,
            InputAction::ToggleAutomap => self.input_mask |= ENGINE_INPUT_TOGGLE_AUTOMAP,
            InputAction::OpenMenu => self.input_mask |= ENGINE_INPUT_OPEN_MENU,
            InputAction::ConfirmMenu => self.input_mask |= ENGINE_INPUT_CONFIRM_MENU,
            InputAction::BackMenu => self.input_mask |= ENGINE_INPUT_BACK_MENU,
            InputAction::WeaponPrev => self.input_mask |= ENGINE_INPUT_WEAPON_PREV,
            InputAction::WeaponNext => self.input_mask |= ENGINE_INPUT_WEAPON_NEXT,
            _ => {}
        }
    }

    pub fn configure_from_map(&mut self, map: &MapData, player: &Player) {
        let mut wall_segments = Vec::new();
        let mut wall_segment_lights = Vec::new();

        for line in &map.lines {
            let is_solid_line = line.right_sidedef != WAD_INVALID_SIDEDEF_INDEX
                && line.left_sidedef == WAD_INVALID_SIDEDEF_INDEX;

            if !is_solid_line {
                continue;
            }

            let Some(start_vertex) = map.vertexes.get(line.start_vertex as usize) else {
                continue;
            };

            let Some(end_vertex) = map.vertexes.get(line.end_vertex as usize) else {
                continue;
            };

            wall_segments.push(start_vertex.x as f32);
            wall_segments.push(start_vertex.y as f32);
            wall_segments.push(end_vertex.x as f32);
            wall_segments.push(end_vertex.y as f32);

            let light_level = map
                .sidedefs
                .get(line.right_sidedef as usize)
                .and_then(|sidedef| map.sectors.get(sidedef.sector as usize))
                .map(|sector| {
                    sector
                        .light_level
                        .clamp(0, FRAMEBUFFER_MAX_CHANNEL_VALUE_U8 as i16) as u8
                })
                .unwrap_or(FRAMEBUFFER_DEFAULT_LIGHT_LEVEL);

            wall_segment_lights.push(light_level);
        }

        engine_set_world(EngineWorldUploadParams {
            wall_segments: &wall_segments,
            wall_segment_lights: &wall_segment_lights,
        });
        engine_set_player_spawn(EngineSetPlayerSpawnParams {
            player_x: player.x,
            player_y: player.y,
            player_angle: player.angle,
        });

        self.engine_player_state.x = player.x;
        self.engine_player_state.y = player.y;
        self.engine_player_state.angle = player.angle;
        self.engine_player_state.health = player.health;
    }

    pub fn sync_player_from_engine(&self, player: &mut Player) {
        player.x = self.engine_player_state.x;
        player.y = self.engine_player_state.y;
        player.angle = self.engine_player_state.angle;
        player.health = self.engine_player_state.health;
    }
}

impl FrameProvider for CFrameProvider {
    fn next_frame(&mut self) -> RgbFrame {
        if !self.engine_initialized {
            engine_init(EngineInitParams {
                width: self.width,
                height: self.height,
            });

            self.engine_initialized = true;
        }

        engine_set_input(self.input_mask);
        engine_step();

        self.input_mask = 0;
        let mut rgb = vec![0u8; self.width * self.height * FRAMEBUFFER_RGB_CHANNELS];

        engine_fill_frame(&mut rgb);

        if let Some(player_state) = engine_get_state() {
            self.engine_player_state = player_state;
        }

        self.tick = self.tick.wrapping_add(1);

        RgbFrame {
            rgb,
            width: self.width,
            height: self.height,
        }
    }
}

#[cfg(test)]
#[path = "../tests/framebuffer/providers_tests.rs"]
mod tests;
