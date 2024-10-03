use std::mem::MaybeUninit;

use crate::prelude::{cvar_manager::CVarManager, game::Game};

#[repr(C)]
pub struct PluginInfo {
	pub api_build_version: u16,
	pub file_name: *const u8,
	pub class_name: *const u8,
	pub plugin_name: *const u8,
	pub plugin_version: *const u8,
	pub plugin_type: u32,
	pub initialize_func: unsafe extern "C" fn() -> usize,
	pub del_func: unsafe extern "C" fn()
}

unsafe impl Send for PluginInfo {}
unsafe impl Sync for PluginInfo {}

extern "C" {
	pub fn bmrs_pluginInit() -> usize;
	pub fn bmrs_pluginUninit();
}

static mut BAKKESMOD: MaybeUninit<BakkesMod> = MaybeUninit::uninit();

pub fn bakkesmod_init(cvm: *mut (), game: *mut ()) {
	unsafe {
		BAKKESMOD = MaybeUninit::new(BakkesMod {
			cvar_manager: CVarManager::from_raw(cvm),
			game: Game::new(game)
		});
	}
}

pub fn bakkesmod_exit() {
	let cvar_manager = &bakkesmod().cvar_manager;
	if let Ok(mut cbs) = cvar_manager.on_changed_cbs.lock() {
		drop(cbs.drain());
	}
	if let Ok(mut cbs) = cvar_manager.notifier_cbs.lock() {
		drop(cbs.drain());
	}

	let game = &bakkesmod().game;
	if let Ok(mut cbs) = game.hook_cbs.lock() {
		drop(cbs.drain());
	}
}

pub fn bakkesmod() -> &'static BakkesMod {
	unsafe { BAKKESMOD.assume_init_ref() }
}

pub struct BakkesMod {
	pub cvar_manager: CVarManager,
	pub game: Game
}
