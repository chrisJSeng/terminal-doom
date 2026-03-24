use std::ffi::{c_char, c_int, c_uint, CString};
use std::ptr;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use crate::constants::{
    FRAMEBUFFER_ENGINE_ARGV_NUL_MSG, FRAMEBUFFER_ENGINE_ARG_IWAD, FRAMEBUFFER_ENGINE_ARG_PROGRAM,
    FRAMEBUFFER_ENGINE_ARG_TURBO, FRAMEBUFFER_ENGINE_ARG_TURBO_VALUE,
    FRAMEBUFFER_ENGINE_STATE_POISONED_MSG, FRAMEBUFFER_ENGINE_WORLD_LIGHT_COUNT_MISMATCH_MSG,
    FRAMEBUFFER_ERR_ENGINE_FILL_FAILED_PREFIX, FRAMEBUFFER_ERR_ENGINE_INIT_FAILED_PREFIX,
    FRAMEBUFFER_ERR_ENGINE_SET_SPAWN_FAILED_PREFIX, FRAMEBUFFER_ERR_ENGINE_SET_WORLD_FAILED_PREFIX,
    FRAMEBUFFER_ERR_HEIGHT_U32_OVERFLOW, FRAMEBUFFER_ERR_WIDTH_U32_OVERFLOW, WAD_CANDIDATES,
};
use crate::types::{
    DoomEngineFillFrameParams, DoomEngineInitParams, DoomEngineSetPlayerSpawnParams,
    DoomEngineState, DoomPlayer, EngineInitParams, EngineKeyEvent, EnginePlayerState,
    EngineSetPlayerSpawnParams, EngineWorldUploadParams, PlayerStateSnapshot,
};

const ENGINE_OK: c_int = 0;
const ENGINE_ERR_INVALID_DIMENSIONS: c_int = 1;
const ENGINE_ERR_OUTPUT_BUFFER: c_int = 2;
const ENGINE_ERR_NOT_INITIALIZED: c_int = 3;

use super::{
    ENGINE_INPUT_BACKWARD, ENGINE_INPUT_BACK_MENU, ENGINE_INPUT_CONFIRM_MENU, ENGINE_INPUT_FIRE,
    ENGINE_INPUT_FORWARD, ENGINE_INPUT_OPEN_MENU, ENGINE_INPUT_STRAFE_LEFT,
    ENGINE_INPUT_STRAFE_RIGHT, ENGINE_INPUT_TOGGLE_AUTOMAP, ENGINE_INPUT_TURN_LEFT,
    ENGINE_INPUT_TURN_RIGHT, ENGINE_INPUT_USE, ENGINE_INPUT_WEAPON_NEXT, ENGINE_INPUT_WEAPON_PREV,
};

const DOOMGENERIC_RESX: usize = 320;
const DOOMGENERIC_RESY: usize = 200;
const DOOM_ANGLE_TO_RADIANS: f32 = std::f32::consts::TAU / 4_294_967_296.0;

const KEY_STRAFE_L: u8 = 0xa0;
const KEY_STRAFE_R: u8 = 0xa1;
const KEY_USE: u8 = 0xa2;
const KEY_FIRE: u8 = 0xa3;
const KEY_ESCAPE: u8 = 27;
const KEY_ENTER: u8 = 13;
const KEY_TAB: u8 = 9;
const KEY_BACKSPACE: u8 = 0x7f;
const KEY_LEFTARROW: u8 = 0xac;
const KEY_UPARROW: u8 = 0xad;
const KEY_RIGHTARROW: u8 = 0xae;
const KEY_DOWNARROW: u8 = 0xaf;

const MAXPLAYERS: usize = 4;
const AMMO_CLIP_INDEX: usize = 0;
const AMMO_SHELL_INDEX: usize = 1;
const FRACUNIT: f32 = 65_536.0;

unsafe extern "C" {
    static mut DG_ScreenBuffer: *mut u32;
    static consoleplayer: c_int;
    static players: [DoomPlayer; MAXPLAYERS];

    fn doomgeneric_Create(argc: c_int, argv: *mut *mut c_char);
    fn doomgeneric_Tick();
}

static ENGINE_STATE: OnceLock<Mutex<DoomEngineState>> = OnceLock::new();

fn engine_state() -> &'static Mutex<DoomEngineState> {
    ENGINE_STATE.get_or_init(|| Mutex::new(DoomEngineState::new()))
}

fn with_engine_state<T>(callback: impl FnOnce(&mut DoomEngineState) -> T) -> T {
    let mut engine_state_guard = engine_state()
        .lock()
        .expect(FRAMEBUFFER_ENGINE_STATE_POISONED_MSG);

    callback(&mut engine_state_guard)
}

pub fn engine_init(params: EngineInitParams) {
    let EngineInitParams { width, height } = params;

    let width_u32 = u32::try_from(width).expect(FRAMEBUFFER_ERR_WIDTH_U32_OVERFLOW);
    let height_u32 = u32::try_from(height).expect(FRAMEBUFFER_ERR_HEIGHT_U32_OVERFLOW);

    let init_params = DoomEngineInitParams {
        width: width_u32,
        height: height_u32,
    };
    let status = doom_engine_init(init_params);

    if status != ENGINE_OK {
        panic!("{FRAMEBUFFER_ERR_ENGINE_INIT_FAILED_PREFIX} {status}");
    }
}

pub fn engine_set_input(input_mask: u32) {
    with_engine_state(|state| state.input_mask = input_mask);
}

pub fn engine_set_world(params: EngineWorldUploadParams<'_>) {
    let EngineWorldUploadParams {
        wall_segments,
        wall_segment_lights,
        ..
    } = params;

    let wall_segment_count = wall_segments.len() / 4;
    let has_matching_light_count = wall_segment_lights.len() == wall_segment_count;

    assert!(
        has_matching_light_count,
        "{}",
        FRAMEBUFFER_ENGINE_WORLD_LIGHT_COUNT_MISMATCH_MSG
    );

    let status = doom_engine_set_world();

    if status != ENGINE_OK {
        panic!("{FRAMEBUFFER_ERR_ENGINE_SET_WORLD_FAILED_PREFIX} {status}");
    }
}

pub fn engine_set_player_spawn(params: EngineSetPlayerSpawnParams) {
    let EngineSetPlayerSpawnParams {
        player_x,
        player_y,
        player_angle,
    } = params;

    let spawn_params = DoomEngineSetPlayerSpawnParams {
        player_x,
        player_y,
        player_angle,
    };
    let status = doom_engine_set_player_spawn(spawn_params);

    if status != ENGINE_OK {
        panic!("{FRAMEBUFFER_ERR_ENGINE_SET_SPAWN_FAILED_PREFIX} {status}");
    }
}

pub fn engine_step() {
    doom_engine_step();
}

pub fn engine_fill_frame(rgb: &mut [u8]) {
    let fill_params = DoomEngineFillFrameParams {
        out_rgb: rgb.as_mut_ptr(),
        out_len: rgb.len(),
    };
    let status = doom_engine_fill_frame(fill_params);

    if status != ENGINE_OK {
        panic!("{FRAMEBUFFER_ERR_ENGINE_FILL_FAILED_PREFIX} {status}");
    }
}

pub fn engine_get_state() -> Option<EnginePlayerState> {
    doom_engine_get_state()
}

fn doom_engine_init(params: DoomEngineInitParams) -> c_int {
    let DoomEngineInitParams { width, height } = params;

    if width == 0 || height == 0 {
        return ENGINE_ERR_INVALID_DIMENSIONS;
    }

    let init_outcome = with_engine_state(|state| {
        state.width = width;
        state.height = height;

        if state.initialized {
            return Ok(None);
        }

        let Some(wad_path) = find_wad_path() else {
            return Err(ENGINE_ERR_NOT_INITIALIZED);
        };

        let (argv_storage, argv_ptrs) = build_argv(&wad_path);

        state.argv_storage = argv_storage;
        state.argv_ptrs = argv_ptrs;
        state.reset_runtime_state();

        Ok(Some((
            state.argv_ptrs.len() as c_int,
            state.argv_ptrs.as_mut_ptr() as *mut *mut c_char,
        )))
    });

    let Some((argc, argv)) = (match init_outcome {
        Ok(value) => value,
        Err(status) => return status,
    }) else {
        return ENGINE_OK;
    };

    unsafe { doomgeneric_Create(argc, argv) };
    with_engine_state(|state| state.initialized = true);

    ENGINE_OK
}

fn doom_engine_set_world() -> c_int {
    ENGINE_OK
}

fn doom_engine_set_player_spawn(params: DoomEngineSetPlayerSpawnParams) -> c_int {
    let DoomEngineSetPlayerSpawnParams {
        player_x,
        player_y,
        player_angle,
    } = params;

    with_engine_state(|state| {
        state.player_state.x = player_x;
        state.player_state.y = player_y;
        state.player_state.angle = player_angle;
    });
    ENGINE_OK
}

fn doom_engine_step() {
    let initialized = with_engine_state(|state| {
        if !state.initialized {
            return false;
        }

        queue_mask_transitions(state);

        state.prev_input_mask = state.input_mask;
        state.input_mask = 0;
        true
    });

    if !initialized {
        return;
    }

    unsafe { doomgeneric_Tick() };
    sync_player_state_from_core();
}

fn doom_engine_fill_frame(params: DoomEngineFillFrameParams) -> c_int {
    let DoomEngineFillFrameParams { out_rgb, out_len } = params;

    let (initialized, width, height) = with_engine_state(|state| {
        let initialized = state.initialized;
        let width = state.width as usize;
        let height = state.height as usize;
        (initialized, width, height)
    });

    let has_invalid_dimensions = width == 0 || height == 0;
    let is_engine_unavailable = !initialized || has_invalid_dimensions;

    if is_engine_unavailable {
        return ENGINE_ERR_NOT_INITIALIZED;
    }

    let required_len = width.saturating_mul(height).saturating_mul(3);

    if out_rgb.is_null() || out_len < required_len {
        return ENGINE_ERR_OUTPUT_BUFFER;
    }

    unsafe {
        if DG_ScreenBuffer.is_null() {
            return ENGINE_ERR_OUTPUT_BUFFER;
        }

        let out_slice = std::slice::from_raw_parts_mut(out_rgb, required_len);

        for y in 0..height {
            let source_y = (y * DOOMGENERIC_RESY / height).min(DOOMGENERIC_RESY - 1);

            for x in 0..width {
                let source_x = (x * DOOMGENERIC_RESX / width).min(DOOMGENERIC_RESX - 1);
                let source_index = source_y * DOOMGENERIC_RESX + source_x;
                let pixel = *DG_ScreenBuffer.add(source_index);
                let destination_index = (y * width + x) * 3;

                out_slice[destination_index] = ((pixel >> 16) & 0xff) as u8;
                out_slice[destination_index + 1] = ((pixel >> 8) & 0xff) as u8;
                out_slice[destination_index + 2] = (pixel & 0xff) as u8;
            }
        }
    }

    ENGINE_OK
}

fn doom_engine_get_state() -> Option<EnginePlayerState> {
    with_engine_state(|state| state.initialized.then_some(state.player_state))
}

fn queue_mask_transitions(state: &mut DoomEngineState) {
    let changed_input_bits = state.prev_input_mask ^ state.input_mask;
    let input_action_bits = [
        ENGINE_INPUT_FORWARD,
        ENGINE_INPUT_BACKWARD,
        ENGINE_INPUT_TURN_LEFT,
        ENGINE_INPUT_TURN_RIGHT,
        ENGINE_INPUT_STRAFE_LEFT,
        ENGINE_INPUT_STRAFE_RIGHT,
        ENGINE_INPUT_FIRE,
        ENGINE_INPUT_USE,
        ENGINE_INPUT_TOGGLE_AUTOMAP,
        ENGINE_INPUT_OPEN_MENU,
        ENGINE_INPUT_CONFIRM_MENU,
        ENGINE_INPUT_BACK_MENU,
        ENGINE_INPUT_WEAPON_PREV,
        ENGINE_INPUT_WEAPON_NEXT,
    ];

    for bit in input_action_bits {
        let did_this_bit_change = changed_input_bits & bit != 0;

        if !did_this_bit_change {
            continue;
        }

        let is_pressed = state.input_mask & bit != 0;
        let key_event = EngineKeyEvent {
            pressed: i32::from(is_pressed),
            key: key_for_input_bit(bit),
        };

        state.key_queue.push_back(key_event);
    }
}

fn key_for_input_bit(bit: u32) -> u8 {
    match bit {
        ENGINE_INPUT_FORWARD => KEY_UPARROW,
        ENGINE_INPUT_BACKWARD => KEY_DOWNARROW,
        ENGINE_INPUT_TURN_LEFT => KEY_LEFTARROW,
        ENGINE_INPUT_TURN_RIGHT => KEY_RIGHTARROW,
        ENGINE_INPUT_STRAFE_LEFT => KEY_STRAFE_L,
        ENGINE_INPUT_STRAFE_RIGHT => KEY_STRAFE_R,
        ENGINE_INPUT_FIRE => KEY_FIRE,
        ENGINE_INPUT_USE => KEY_USE,
        ENGINE_INPUT_TOGGLE_AUTOMAP => KEY_TAB,
        ENGINE_INPUT_OPEN_MENU => KEY_ESCAPE,
        ENGINE_INPUT_CONFIRM_MENU => KEY_ENTER,
        ENGINE_INPUT_BACK_MENU => KEY_BACKSPACE,
        ENGINE_INPUT_WEAPON_PREV => b'[',
        ENGINE_INPUT_WEAPON_NEXT => b']',
        _ => KEY_UPARROW,
    }
}

fn find_wad_path() -> Option<String> {
    WAD_CANDIDATES
        .iter()
        .copied()
        .find(|wad_candidate_path| std::path::Path::new(wad_candidate_path).exists())
        .map(str::to_owned)
}

fn build_argv(wad_path: &str) -> (Vec<CString>, Box<[usize]>) {
    let engine_arguments = [
        FRAMEBUFFER_ENGINE_ARG_PROGRAM,
        FRAMEBUFFER_ENGINE_ARG_IWAD,
        wad_path,
        FRAMEBUFFER_ENGINE_ARG_TURBO,
        FRAMEBUFFER_ENGINE_ARG_TURBO_VALUE,
    ];

    let argument_storage: Vec<CString> = engine_arguments
        .into_iter()
        .map(|arg| CString::new(arg).expect(FRAMEBUFFER_ENGINE_ARGV_NUL_MSG))
        .collect();

    let argument_pointers = argument_storage
        .iter()
        .map(|arg| arg.as_ptr() as usize)
        .collect::<Vec<_>>()
        .into_boxed_slice();

    (argument_storage, argument_pointers)
}

fn sync_player_state_from_core() {
    unsafe {
        if consoleplayer < 0 || consoleplayer as usize >= MAXPLAYERS {
            return;
        }

        let player = &players[consoleplayer as usize];

        if player.mo.is_null() {
            return;
        }

        let mobj = &*player.mo;

        let snapshot = PlayerStateSnapshot {
            x: mobj.x as f32 / FRACUNIT,
            y: mobj.y as f32 / FRACUNIT,
            angle: mobj.angle as f32 * DOOM_ANGLE_TO_RADIANS,
            health: player.health.clamp(0, u8::MAX as i32) as u8,
            armor: player.armorpoints.clamp(0, u16::MAX as i32) as u16,
            bullets: player.ammo[AMMO_CLIP_INDEX].clamp(0, u16::MAX as i32) as u16,
            shells: player.ammo[AMMO_SHELL_INDEX].clamp(0, u16::MAX as i32) as u16,
        };

        with_engine_state(|state| snapshot.apply_to(&mut state.player_state));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn DG_Init() {}

#[unsafe(no_mangle)]
pub extern "C" fn DG_DrawFrame() {}

#[unsafe(no_mangle)]
pub extern "C" fn DG_SleepMs(milliseconds: c_uint) {
    std::thread::sleep(Duration::from_millis(milliseconds as u64));
}

#[unsafe(no_mangle)]
pub extern "C" fn DG_GetTicksMs() -> c_uint {
    with_engine_state(|state| state.start_time.elapsed().as_millis() as c_uint)
}

#[unsafe(no_mangle)]
pub extern "C" fn DG_GetKey(pressed: *mut c_int, key: *mut u8) -> c_int {
    if pressed.is_null() || key.is_null() {
        return 0;
    }

    let event = with_engine_state(|state| state.key_queue.pop_front());
    let Some(event) = event else {
        return 0;
    };

    unsafe {
        ptr::write(pressed, event.pressed);
        ptr::write(key, event.key);
    }

    1
}

#[unsafe(no_mangle)]
pub extern "C" fn DG_SetWindowTitle(_window_title: *const c_char) {}
