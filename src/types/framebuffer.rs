pub struct RgbFrame {
    pub rgb: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct EnginePlayerState {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub health: u8,
    pub armor: u16,
    pub bullets: u16,
    pub shells: u16,
}

pub struct PlayerStateSnapshot {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub health: u8,
    pub armor: u16,
    pub bullets: u16,
    pub shells: u16,
}

impl PlayerStateSnapshot {
    pub fn apply_to(self, player_state: &mut EnginePlayerState) {
        player_state.x = self.x;
        player_state.y = self.y;
        player_state.angle = self.angle;
        player_state.health = self.health;
        player_state.armor = self.armor;
        player_state.bullets = self.bullets;
        player_state.shells = self.shells;
    }
}

pub struct EngineWorldUploadParams<'a> {
    pub wall_segments: &'a [f32],
    pub wall_segment_lights: &'a [u8],
}

pub struct CFrameProvider {
    pub width: usize,
    pub height: usize,
    pub tick: u32,
    pub input_mask: u32,
    pub engine_initialized: bool,
    pub engine_player_state: EnginePlayerState,
}
