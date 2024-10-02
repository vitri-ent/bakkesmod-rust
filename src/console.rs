use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::internal;
use crate::wrappers::cvar::CVar;

pub fn console_print(text: &str) {
	internal::bakkesmod().cvar_manager().log(text);
}

pub fn register_notifier<F: FnMut(Vec<String>) + 'static>(name: &str, callback: F) {
	internal::bakkesmod().cvar_manager().register_notifier(name, callback);
}

pub fn remove_notifier(name: &str) -> bool {
	internal::bakkesmod().cvar_manager().remove_notifier(name)
}

pub fn register_cvar(name: &str) -> CVar {
	internal::bakkesmod().cvar_manager().register(name)
}

pub fn remove_cvar(name: &str) -> bool {
	internal::bakkesmod().cvar_manager().remove(name)
}

pub fn get_cvar(name: &str) -> Option<CVar> {
	internal::bakkesmod().cvar_manager().get(name)
}

pub fn execute_command(command: &str, log: bool) {
	internal::bakkesmod().cvar_manager().execute(command, log);
}

pub fn get_bind_string_for_key(key: &str) -> String {
	let c_string = CString::new(key).unwrap();
	let c_string: *const c_char = c_string.as_ptr();

	let result: *const c_char = 0 as *const c_char;
	let result_ptr: *const *const c_char = &result as *const *const c_char;

	unsafe {
		GetBindStringForKey(c_string, result_ptr);
		let result = *result_ptr;
		let c_result = CStr::from_ptr(result);
		match c_result.to_str() {
			Ok(s) => String::from(s),
			Err(_) => String::new()
		}
	}
}

pub fn set_bind(key: &str, command: &str) {
	let c_key = CString::new(key).unwrap();
	let c_key: *const c_char = c_key.as_ptr();
	let c_command = CString::new(command).unwrap();
	let c_command: *const c_char = c_command.as_ptr();

	unsafe {
		SetBind(c_key, c_command);
	}
}

pub fn get_alias(alias: &str) -> String {
	let c_string = CString::new(alias).unwrap();
	let c_string: *const c_char = c_string.as_ptr();

	let result: *const c_char = 0 as *const c_char;
	let result_ptr: *const *const c_char = &result as *const *const c_char;

	unsafe {
		GetAlias(c_string, result_ptr);
		let result = *result_ptr;
		let c_result = CStr::from_ptr(result);
		match c_result.to_str() {
			Ok(s) => String::from(s),
			Err(_) => String::new()
		}
	}
}

pub fn set_alias(key: &str, script: &str) {
	let c_key = CString::new(key).unwrap();
	let c_key: *const c_char = c_key.as_ptr();
	let c_script = CString::new(script).unwrap();
	let c_script: *const c_char = c_script.as_ptr();

	unsafe {
		SetBind(c_key, c_script);
	}
}

pub fn backup_cfg(path: &str) {
	let c_path = CString::new(path).unwrap();
	let c_path: *const c_char = c_path.as_ptr();
	unsafe {
		BackupCfg(c_path);
	}
}

pub fn backup_binds(path: &str) {
	let c_path = CString::new(path).unwrap();
	let c_path: *const c_char = c_path.as_ptr();
	unsafe {
		BackupBinds(c_path);
	}
}

pub fn load_cfg(path: &str) {
	let c_path = CString::new(path).unwrap();
	let c_path: *const c_char = c_path.as_ptr();
	unsafe {
		LoadCfg(c_path);
	}
}

#[allow(unused)]
extern "C" {
	fn LogConsole(id: u64, text: *const c_char);
	fn RegisterNotifier(id: u64, user_data: usize, cvar: *const c_char, callback: usize, description: *const c_char, permissions: u8);
	fn RemoveNotifier(id: u64, cvar: *const c_char) -> bool;
	fn RegisterCVar(
		id: u64,
		cvar: *const c_char,
		default_value: *const c_char,
		desc: *const c_char,
		searchable: bool,
		has_min: bool,
		min: f32,
		has_max: bool,
		max: f32,
		save_to_cfg: bool
	) -> usize;
	fn RemoveCvar(id: u64, cvar: *const c_char) -> bool;
	fn GetCVar(name: *const c_char) -> usize;

	fn ExecuteCommand(command: *const c_char, log: bool);
	fn GetBindStringForKey(key: *const c_char, result: *const *const c_char);
	fn SetBind(key: *const c_char, command: *const c_char);
	fn GetAlias(alias: *const c_char, result: *const *const c_char);
	fn SetAlias(key: *const c_char, script: *const c_char);
	fn BackupCfg(path: *const c_char);
	fn BackupBinds(path: *const c_char);
	fn LoadCfg(path: *const c_char);

	fn CVar_AddOnValueChanged(p_cvar: usize, id: u64, user_data: usize, callback: usize);
	fn CVar_RemoveOnValueChanged(p_cvar: usize, id: u64);
}
