use std::collections::VecDeque;
use std::ffi::{c_char, c_int, c_void, CString};
use std::time::Instant;

use crate::constants::FRAMEBUFFER_ENGINE_KEY_QUEUE_CAPACITY;

use super::EnginePlayerState;

pub struct EngineInitParams {
    pub width: usize,
    pub height: usize,
}

pub struct EngineSetPlayerSpawnParams {
    pub player_x: f32,
    pub player_y: f32,
    pub player_angle: f32,
}

pub(crate) struct DoomEngineInitParams {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

pub(crate) struct DoomEngineSetPlayerSpawnParams {
    pub(crate) player_x: f32,
    pub(crate) player_y: f32,
    pub(crate) player_angle: f32,
}

pub(crate) struct DoomEngineFillFrameParams {
    pub(crate) out_rgb: *mut u8,
    pub(crate) out_len: usize,
}

#[repr(C)]
pub(crate) struct DoomTicCmd {
    pub(crate) forwardmove: i8,
    pub(crate) sidemove: i8,
    pub(crate) angleturn: i16,
    pub(crate) chatchar: u8,
    pub(crate) buttons: u8,
    pub(crate) consistancy: u8,
    pub(crate) buttons2: u8,
    pub(crate) inventory: i32,
    pub(crate) lookfly: u8,
    pub(crate) arti: u8,
}

#[repr(C)]
pub(crate) struct DoomPspDef {
    pub(crate) state: *mut c_void,
    pub(crate) tics: i32,
    pub(crate) sx: i32,
    pub(crate) sy: i32,
}

#[repr(C)]
pub(crate) struct DoomThinker {
    pub(crate) prev: *mut c_void,
    pub(crate) next: *mut c_void,
    pub(crate) function: *mut c_void,
}

#[repr(C)]
pub(crate) struct DoomMobj {
    pub(crate) thinker: DoomThinker,
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) z: i32,
    pub(crate) snext: *mut DoomMobj,
    pub(crate) sprev: *mut DoomMobj,
    pub(crate) angle: u32,
    pub(crate) sprite: i32,
    pub(crate) frame: i32,
    pub(crate) bnext: *mut DoomMobj,
    pub(crate) bprev: *mut DoomMobj,
    pub(crate) subsector: *mut c_void,
    pub(crate) floorz: i32,
    pub(crate) ceilingz: i32,
    pub(crate) radius: i32,
    pub(crate) height: i32,
    pub(crate) momx: i32,
    pub(crate) momy: i32,
    pub(crate) momz: i32,
    pub(crate) validcount: i32,
    pub(crate) mobj_type: i32,
    pub(crate) info: *mut c_void,
    pub(crate) tics: i32,
    pub(crate) state: *mut c_void,
    pub(crate) flags: i32,
    pub(crate) health: i32,
}

#[repr(C)]
pub(crate) struct DoomPlayer {
    pub(crate) mo: *mut DoomMobj,
    pub(crate) playerstate: i32,
    pub(crate) cmd: DoomTicCmd,
    pub(crate) viewz: i32,
    pub(crate) viewheight: i32,
    pub(crate) deltaviewheight: i32,
    pub(crate) bob: i32,
    pub(crate) health: i32,
    pub(crate) armorpoints: i32,
    pub(crate) armortype: i32,
    pub(crate) powers: [i32; 6],
    pub(crate) cards: [i32; 6],
    pub(crate) backpack: i32,
    pub(crate) frags: [i32; 4],
    pub(crate) readyweapon: i32,
    pub(crate) pendingweapon: i32,
    pub(crate) weaponowned: [i32; 9],
    pub(crate) ammo: [i32; 4],
    pub(crate) maxammo: [i32; 4],
    pub(crate) attackdown: i32,
    pub(crate) usedown: i32,
    pub(crate) cheats: i32,
    pub(crate) refire: i32,
    pub(crate) killcount: i32,
    pub(crate) itemcount: i32,
    pub(crate) secretcount: i32,
    pub(crate) message: *mut c_char,
    pub(crate) damagecount: i32,
    pub(crate) bonuscount: i32,
    pub(crate) attacker: *mut DoomMobj,
    pub(crate) extralight: i32,
    pub(crate) fixedcolormap: i32,
    pub(crate) colormap: i32,
    pub(crate) psprites: [DoomPspDef; 2],
    pub(crate) didsecret: i32,
}

#[derive(Clone, Copy)]
pub(crate) struct EngineKeyEvent {
    pub(crate) pressed: c_int,
    pub(crate) key: u8,
}

pub(crate) struct DoomEngineState {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) input_mask: u32,
    pub(crate) prev_input_mask: u32,
    pub(crate) initialized: bool,
    pub(crate) player_state: EnginePlayerState,
    pub(crate) key_queue: VecDeque<EngineKeyEvent>,
    pub(crate) argv_storage: Vec<CString>,
    pub(crate) argv_ptrs: Box<[usize]>,
    pub(crate) start_time: Instant,
}

impl DoomEngineState {
    pub(crate) fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            input_mask: 0,
            prev_input_mask: 0,
            initialized: false,
            player_state: EnginePlayerState::spawn_default(),
            key_queue: VecDeque::with_capacity(FRAMEBUFFER_ENGINE_KEY_QUEUE_CAPACITY),
            argv_storage: Vec::new(),
            argv_ptrs: Box::new([]),
            start_time: Instant::now(),
        }
    }

    pub(crate) fn reset_runtime_state(&mut self) {
        self.player_state = EnginePlayerState::spawn_default();
        self.input_mask = 0;
        self.prev_input_mask = 0;
        self.key_queue.clear();
        self.start_time = Instant::now();
    }
}
