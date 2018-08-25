use link::{Inventory, Link};

#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct Momentum {
    _p0: [u8; 0x4F8],
    pub link_momentum: Vec3,
}

#[repr(C)]
pub struct GameInfo {
    pub link: Link,                                  // 804061c0
    _p0: [u8; 124],                                  // 804061e0
    pub inventory: Inventory,                        // 8040625C
    _p1: [u8; 0x55C8],                               // 804062B0
    pub momentum_ptr: Option<&'static mut Momentum>, // 8040B878
}

#[repr(C)]
pub struct GlobalCounters {
    pub game_counter: u32,
    pub game_counter2: u32,
    pub non_menu_counter: u32,
}

#[repr(C)]
pub struct ZelAudio {
    _p0: [u8; 0xF08],
    pub link_debug_ptr: Option<&'static mut LinkDebug>,
}

#[repr(C)]
pub struct LinkDebug {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    _p0: [u8; 0xA],
    pub facing: u16,
    _p1: [u8; 0x44],
    pub speed: f32,
}

extern "C" {
    #[link_name = "g_Counter"]
    pub static mut GLOBAL_COUNTERS: GlobalCounters;
    #[link_name = "g_mDoAud_zelAudio"]
    pub static mut ZEL_AUDIO: ZelAudio;
    #[link_name = "g_dComIfG_gameInfo"]
    pub static mut GAME_INFO: GameInfo;
}

pub fn get_frame_count() -> u32 {
    unsafe { GLOBAL_COUNTERS.game_counter }
}

pub fn get_link_debug() -> Option<&'static mut LinkDebug> {
    unsafe {
        if let Some(ref mut link_debug) = ZEL_AUDIO.link_debug_ptr {
            Some(*link_debug)
        } else {
            None
        }
    }
}

pub fn get_link_momentum() -> Option<&'static mut Momentum> {
    unsafe {
        if let Some(ref mut link_momentum) = GAME_INFO.momentum_ptr {
            Some(*link_momentum)
        } else {
            None
        }
    }
}
