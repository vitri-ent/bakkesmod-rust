use crate::internal;
use crate::wrappers::CVar;

pub fn console_print(text: &str) {
	internal::bakkesmod().cvar_manager.log(text);
}

pub fn register_notifier<F: FnMut(Vec<String>) + 'static>(name: &str, callback: F) {
	internal::bakkesmod().cvar_manager.register_notifier(name, callback);
}

pub fn remove_notifier(name: &str) -> bool {
	internal::bakkesmod().cvar_manager.remove_notifier(name)
}

pub fn register_cvar(name: &str) -> CVar {
	internal::bakkesmod().cvar_manager.register(name)
}

pub fn remove_cvar(name: &str) -> bool {
	internal::bakkesmod().cvar_manager.remove(name)
}

pub fn get_cvar(name: &str) -> Option<CVar> {
	internal::bakkesmod().cvar_manager.get(name)
}

pub fn execute_command(command: &str, log: bool) {
	internal::bakkesmod().cvar_manager.execute(command, log);
}

pub fn get_bind_string_for_key(key: &str) -> String {
	internal::bakkesmod().cvar_manager.get_bind(key)
}

pub fn set_bind(key: &str, command: &str) {
	internal::bakkesmod().cvar_manager.set_bind(key, command);
}

pub fn get_alias(alias: &str) -> String {
	internal::bakkesmod().cvar_manager.get_alias(alias)
}

pub fn set_alias(key: &str, script: &str) {
	internal::bakkesmod().cvar_manager.set_alias(key, script);
}

pub fn backup_cfg(path: &str) {
	internal::bakkesmod().cvar_manager.backup_cfg(path);
}

pub fn backup_binds(path: &str) {
	internal::bakkesmod().cvar_manager.backup_binds(path);
}

pub fn load_cfg(path: &str) {
	internal::bakkesmod().cvar_manager.load_cfg(path);
}
