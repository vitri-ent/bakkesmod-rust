use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::mem::ManuallyDrop;
use std::os::raw::c_char;
use std::ptr::{self, addr_of, addr_of_mut};
use std::sync::Mutex;

use super::cvar::{CVar, OldCVar};
use super::structs::BmrsString;

pub struct CVarManager {
	ptr: *mut (),
	pub(crate) on_changed_cbs: Mutex<HashMap<String, Vec<Box<Box<dyn FnMut(String, &CVar)>>>>>,
	pub(crate) notifier_cbs: Mutex<HashMap<BmrsString, Box<Box<dyn FnMut(Vec<String>)>>>>
}

impl CVarManager {
	pub(crate) fn from_raw(ptr: *mut ()) -> Self {
		Self {
			ptr,
			on_changed_cbs: Mutex::new(HashMap::new()),
			notifier_cbs: Mutex::new(HashMap::new())
		}
	}

	pub fn execute(&self, command: &str, log: bool) {
		unsafe {
			bmrsCVarManager_execute_command(self.ptr, &command.into(), log);
		}
	}

	pub fn register_notifier<F>(&self, cvar: &str, notifier: F)
	where
		F: FnMut(Vec<String>) + 'static
	{
		let cb = Box::new(Box::new(notifier) as Box<dyn FnMut(Vec<String>)>);
		let cvar: BmrsString = cvar.into();
		unsafe {
			bmrsCVarManager_register_notifier(self.ptr, &cvar, Self::register_notifier_callback, &*cb as *const _ as *mut (), &"".into(), 0);
		};
		self.notifier_cbs.lock().unwrap().insert(cvar, cb);
	}

	pub fn remove_notifier(&self, cvar: &str) -> bool {
		let cvar: BmrsString = cvar.into();
		drop(self.notifier_cbs.lock().unwrap().remove(&cvar));
		unsafe { bmrsCVarManager_remove_notifier(self.ptr, &cvar) }
	}

	pub fn register(&self, name: &str) -> CVar {
		CVar::new(unsafe { bmrsCVarManager_register_cvar(self.ptr, &name.into(), &"".into(), &"".into(), true, false, 0.0, false, 0.0, true) })
	}

	pub fn log(&self, message: impl Into<BmrsString>) {
		unsafe {
			bmrsCVarManager_log(self.ptr, &message.into());
		}
	}

	pub fn get(&self, cvar: &str) -> Option<CVar> {
		let cvar = unsafe { bmrsCVarManager_get_cvar(self.ptr, &cvar.into()) };
		if cvar.is_null() { None } else { Some(CVar::new(cvar)) }
	}

	pub fn remove(&self, cvar: &str) -> bool {
		unsafe { bmrsCVarManager_remove_cvar(self.ptr, &cvar.into()) }
	}

	extern "C" fn register_notifier_callback(args: *const BmrsString, n_args: usize, aux: *mut ()) {
		let mut args_vec = Vec::with_capacity(n_args);
		for i in 0..n_args {
			args_vec.push(unsafe { ptr::read(args.add(i)) }.to_string());
		}

		let mut closure = unsafe { Box::from_raw(aux as *mut Box<dyn Fn(Vec<String>)>) };
		(*closure)(args_vec);
		let _ = Box::into_raw(closure);
	}
}

extern "C" {
	fn bmrsCVarManager_execute_command(this: *mut (), command: *const BmrsString, log: bool);
	fn bmrsCVarManager_register_notifier(
		this: *mut (),
		cvar: *const BmrsString,
		notifier: extern "C" fn(args: *const BmrsString, n_args: usize, aux: *mut ()),
		aux: *mut (),
		description: *const BmrsString,
		permissions: u8
	);
	fn bmrsCVarManager_remove_notifier(this: *mut (), cvar: *const BmrsString) -> bool;
	fn bmrsCVarManager_register_cvar(
		this: *mut (),
		name: *const BmrsString,
		default_value: *const BmrsString,
		description: *const BmrsString,
		searchable: bool,
		has_min: bool,
		min: f32,
		has_max: bool,
		max: f32,
		save_to_cfg: bool
	) -> *mut ();
	fn bmrsCVarManager_remove_cvar(this: *mut (), cvar: *const BmrsString) -> bool;
	fn bmrsCVarManager_log(this: *mut (), message: *const BmrsString);
	fn bmrsCVarManager_get_cvar(this: *mut (), cvar: *const BmrsString) -> *mut ();
}
