#[derive(Clone, Copy, Debug)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub health: u8,
}

use super::MapData;

pub struct GameState {
    pub player: Player,
    pub map: MapData,
}

pub struct GameStateInit {
    pub map: MapData,
    pub player: Player,
}

#[cfg(test)]
pub struct CollidesWithMapParams<'a> {
    pub player: &'a Player,
    pub map: &'a MapData,
}
