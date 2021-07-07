#![feature(llvm_asm)]
#![feature(destructuring_assignment)]
#![feature(maybe_uninit_ref)]
#![allow(clippy::missing_safety_doc)]

use std::ptr::null_mut;
use std::time::Instant;

use log::*;
use log::LevelFilter;

use crate::cheat::CHEAT;
use crate::config::CONFIG;
use crate::decryption::DECRYPTION;
use crate::gamedata::GAMEDATA;
use crate::gui::GUI;
use crate::memory::MEMORY;
use crate::overlay::ImguiOverlay;

pub mod cheat;
pub mod gui;
pub mod util;
pub mod memory;
pub mod decryption;
pub mod logger;
pub mod offsets;
pub mod sdk;
pub mod math;
pub mod gamedata;
pub mod overlay;
pub mod hacks;
pub mod config;
pub mod fonts;
pub mod prediction;

pub static VERSION: &str = concat!(env!("GIT_BRANCH"), "/", env!("GIT_HASH"), env!("GIT_MODIFIED_STR"));
// pub static DEBUG: bool = cfg!(debug_assertations);
pub static DEBUG: bool = true;

#[no_mangle]
pub unsafe extern "C" fn on_load() {
    logger::Logger::init(LevelFilter::Trace);

    MEMORY.init_default();
    GUI.init_default();
    GAMEDATA.init_default();
    DECRYPTION.init_default();
    CHEAT.init_default();
    CONFIG.init_default();
}

#[no_mangle]
pub unsafe extern "C" fn on_imgui_init(ctx: *mut imgui::sys::ImGuiContext) {
    let mut ctx = imgui::Context::from_raw(ctx);
    fonts::init_fonts(&mut ctx, 1.0);
}

#[no_mangle]
pub unsafe extern "C" fn on_frame(ctx: *mut imgui::sys::ImGuiContext) {
    if let Err(e) = std::panic::catch_unwind(|| {
        let start = Instant::now();

        static mut IMGUI_CONTEXT: Option<imgui::Context> = None;
        let ctx = IMGUI_CONTEXT.get_or_insert_with(|| imgui::Context::from_raw(ctx));
        ctx.io_mut().want_capture_mouse = GUI.is_open();

        let ui = imgui::Ui { ctx, font_atlas: None };

        CHEAT.get_mut().tick();
        ImguiOverlay::build(&ui, |overlay| {
            CHEAT.get_mut().render(&overlay);
        });
        GUI.get_mut().render(&ui);

        CHEAT.get_mut().last_frame_time = start.elapsed()
    }) {
        error!("Panic during frame: {:?}", e);
    }
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq)]
pub enum InputType {
    KeyDown = 0,
    KeyUp = 1,
}

#[no_mangle]
pub unsafe extern "C" fn on_input_event(input_type: InputType, key: i32) {
    CHEAT.get_mut().handle_input(input_type, key);
}

#[link(name = "framework")]
extern "C" {
    fn unload_cheat();
    fn show_memory_editor();
}
