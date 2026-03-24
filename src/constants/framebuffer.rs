pub const FRAMEBUFFER_TEST_WIDTH: usize = 160;
pub const FRAMEBUFFER_TEST_HEIGHT: usize = 100;
pub const FRAMEBUFFER_COLOR_MAX_F32: f32 = 255.0;
pub const FRAMEBUFFER_RGB_CHANNELS: usize = 3;
pub const FRAMEBUFFER_MAX_CHANNEL_VALUE_U8: u8 = 255;
pub const FRAMEBUFFER_DEFAULT_LIGHT_LEVEL: u8 = 160;

pub const FRAMEBUFFER_ENGINE_KEY_QUEUE_CAPACITY: usize = 128;
pub const FRAMEBUFFER_ENGINE_STATE_POISONED_MSG: &str = "doom engine state poisoned";
pub const FRAMEBUFFER_ENGINE_WORLD_LIGHT_COUNT_MISMATCH_MSG: &str =
    "wall segment lights must match segment count";
pub const FRAMEBUFFER_ENGINE_ARGV_NUL_MSG: &str = "doom argv contained interior NUL";
pub const FRAMEBUFFER_ENGINE_ARG_PROGRAM: &str = "doom-terminal";
pub const FRAMEBUFFER_ENGINE_ARG_IWAD: &str = "-iwad";
pub const FRAMEBUFFER_ENGINE_ARG_TURBO: &str = "-turbo";
pub const FRAMEBUFFER_ENGINE_ARG_TURBO_VALUE: &str = "100";
