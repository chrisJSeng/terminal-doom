pub trait FrameProvider {
    fn next_frame(&mut self) -> crate::types::RgbFrame;
}

const ENGINE_INPUT_FORWARD: u32 = 1;
const ENGINE_INPUT_BACKWARD: u32 = 2;
const ENGINE_INPUT_TURN_LEFT: u32 = 4;
const ENGINE_INPUT_TURN_RIGHT: u32 = 8;
const ENGINE_INPUT_STRAFE_LEFT: u32 = 16;
const ENGINE_INPUT_STRAFE_RIGHT: u32 = 32;
const ENGINE_INPUT_FIRE: u32 = 64;
const ENGINE_INPUT_USE: u32 = 128;
const ENGINE_INPUT_TOGGLE_AUTOMAP: u32 = 256;
const ENGINE_INPUT_OPEN_MENU: u32 = 512;
const ENGINE_INPUT_CONFIRM_MENU: u32 = 1024;
const ENGINE_INPUT_BACK_MENU: u32 = 2048;
const ENGINE_INPUT_WEAPON_PREV: u32 = 4096;
const ENGINE_INPUT_WEAPON_NEXT: u32 = 8192;

mod c_backend;

mod providers;
