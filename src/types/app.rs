use std::collections::HashMap;
use std::time::Instant;

use super::{GameState, MapBounds, MapData, RenderState, TerminalRenderer};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InputAction {
    Quit,
    MoveForward,
    MoveBackward,
    StrafeLeft,
    StrafeRight,
    RotateLeft,
    RotateRight,
    Fire,
    Use,
    ToggleAutomap,
    OpenMenu,
    ConfirmMenu,
    BackMenu,
    WeaponPrev,
    WeaponNext,
}

pub struct StatusTextParams<'a> {
    pub backend_label: &'a str,
    pub source_label: &'a str,
    pub collision_flag: &'a str,
    pub pickup_flag: &'a str,
    pub health: u8,
    pub armor: u16,
    pub bullets: u16,
    pub shells: u16,
    pub player_alive: &'a str,
    pub world_x: f32,
    pub world_y: f32,
    pub angle_degrees: u16,
    pub zoom: f32,
    pub max_width: usize,
}

pub struct AppBootstrap {
    pub game_state: GameState,
    pub render_state: RenderState,
    pub renderer: TerminalRenderer,
}

pub struct BoolLabelParams {
    pub flag: bool,
    pub false_label: &'static str,
    pub true_label: &'static str,
}

pub struct RuntimeStatusTextParams<'a> {
    pub game_state: &'a GameState,
    pub last_collision: bool,
    pub last_pickup: bool,
    pub render_state: &'a RenderState,
    pub max_width: usize,
}

pub struct InputState {
    pub(crate) held_actions: HashMap<InputAction, Instant>,
}

pub(crate) struct MapPlayerSpawnParams<'a> {
    pub(crate) map_data: &'a MapData,
    pub(crate) bounds: MapBounds,
}

pub(crate) struct ApplyExternalFrameInputParams<'a> {
    pub(crate) input_state: &'a InputState,
    pub(crate) pressed_actions: &'a [InputAction],
    pub(crate) render_state: &'a mut RenderState,
}

pub(crate) struct ApplyExternalEngineInputParams<'a> {
    pub(crate) action: InputAction,
    pub(crate) render_state: &'a mut RenderState,
}

#[cfg(test)]
pub struct TryMovePlayerWithCollisionParams<'a> {
    pub game_state: &'a mut GameState,
    pub distance: f32,
}

pub(crate) struct InputStateUpdateParams<'a> {
    pub pressed: &'a [InputAction],
    pub now: std::time::Instant,
}

pub(crate) struct ResolvedOpposedActionParams {
    pub pair: (bool, bool),
    pub first: InputAction,
    pub second: InputAction,
}

pub(crate) struct ProcessKeyEventParams<'a> {
    pub(crate) key: crossterm::event::KeyEvent,
    pub(crate) pressed: &'a mut Vec<InputAction>,
    pub(crate) released: &'a mut Vec<InputAction>,
}
