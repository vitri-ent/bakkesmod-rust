use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::mem::ManuallyDrop;
use std::os::raw::c_char;
use std::ptr;
use std::sync::Mutex;

use super::cvar_manager::CVarManager;
use super::structs::{BmrsString, LinearColor};
use crate::internal;

pub struct CVar {
	ptr: *mut ()
}

unsafe impl Send for CVar {}
unsafe impl Sync for CVar {}

impl CVar {
	pub(crate) fn new(ptr: *mut ()) -> Self {
		CVar { ptr }
	}

	pub fn get_name(&self) -> String {
		unsafe { bmrsCVar_get_name(self.ptr) }.to_string()
	}

	pub fn on_changed<F>(&self, handler: F)
	where
		F: FnMut(String, &CVar) + 'static
	{
		let cb = Box::new(Box::new(handler) as Box<dyn FnMut(String, &CVar)>);
		unsafe {
			bmrsCVar_add_on_value_changed(self.ptr, Self::on_changed_callback, &*cb as *const _ as *mut ());
		};
		internal::bakkesmod()
			.cvar_manager
			.on_changed_cbs
			.lock()
			.unwrap()
			.entry(self.get_name())
			.or_default()
			.push(cb);
	}

	pub fn remove_on_changed(&self) {
		unsafe {
			bmrsCVar_remove_on_value_changed(self.ptr);
		}
		drop(
			internal::bakkesmod()
				.cvar_manager
				.on_changed_cbs
				.lock()
				.unwrap()
				.get_mut(&self.get_name())
				.map(|c| c.drain(..))
		);
	}

	pub fn notify(&self) {
		unsafe {
			bmrsCVar_notify(self.ptr);
		}
	}

	pub fn get<T: CVarValue>(&self) -> T {
		unsafe { T::get(self.ptr) }
	}

	pub fn set<T: CVarValueSetter>(&self, value: T) {
		unsafe { T::set(&value, self.ptr) }
	}

	extern "C" fn on_changed_callback(old_value: BmrsString, this: *mut (), aux: *mut ()) {
		let cvar = CVar::new(this);
		let mut closure = unsafe { Box::from_raw(aux as *mut Box<dyn FnMut(String, &CVar)>) };
		(*closure)(old_value.to_string(), &cvar);
		let _ = Box::into_raw(closure);
	}
}

impl Drop for CVar {
	fn drop(&mut self) {
		unsafe { bmrsCVar_drop(self.ptr) }
	}
}

pub trait CVarValueSetter {
	unsafe fn set(&self, cvar: *mut ());
}
pub trait CVarValue: CVarValueSetter {
	unsafe fn get(cvar: *mut ()) -> Self;
}

impl CVarValueSetter for i32 {
	unsafe fn set(&self, cvar: *mut ()) {
		unsafe { bmrsCVar_set_int_value(cvar, *self) }
	}
}
impl CVarValue for i32 {
	unsafe fn get(cvar: *mut ()) -> Self {
		unsafe { bmrsCVar_get_int_value(cvar) }
	}
}
impl CVarValueSetter for f32 {
	unsafe fn set(&self, cvar: *mut ()) {
		unsafe { bmrsCVar_set_float_value(cvar, *self) }
	}
}
impl CVarValue for f32 {
	unsafe fn get(cvar: *mut ()) -> Self {
		unsafe { bmrsCVar_get_float_value(cvar) }
	}
}
impl CVarValueSetter for String {
	unsafe fn set(&self, cvar: *mut ()) {
		unsafe { bmrsCVar_set_string_value(cvar, &self.as_str().into()) }
	}
}
impl CVarValueSetter for &str {
	unsafe fn set(&self, cvar: *mut ()) {
		unsafe { bmrsCVar_set_string_value(cvar, &(*self).into()) }
	}
}
impl CVarValue for String {
	unsafe fn get(cvar: *mut ()) -> Self {
		unsafe { bmrsCVar_get_string_value(cvar) }.to_string()
	}
}
impl CVarValueSetter for bool {
	unsafe fn set(&self, cvar: *mut ()) {
		<i32 as CVarValueSetter>::set(&1, cvar)
	}
}
impl CVarValue for bool {
	unsafe fn get(cvar: *mut ()) -> Self {
		unsafe { bmrsCVar_get_bool_value(cvar) }
	}
}
impl CVarValueSetter for LinearColor {
	unsafe fn set(&self, cvar: *mut ()) {
		unsafe { bmrsCVar_set_color_value(cvar, *self) }
	}
}
impl CVarValue for LinearColor {
	unsafe fn get(cvar: *mut ()) -> Self {
		unsafe { bmrsCVar_get_color_value(cvar) }
	}
}

extern "C" {
	fn bmrsCVar_get_name(this: *mut ()) -> BmrsString;
	fn bmrsCVar_get_int_value(this: *mut ()) -> i32;
	fn bmrsCVar_get_float_value(this: *mut ()) -> f32;
	fn bmrsCVar_get_bool_value(this: *mut ()) -> bool;
	fn bmrsCVar_get_color_value(this: *mut ()) -> LinearColor;
	fn bmrsCVar_get_string_value(this: *mut ()) -> BmrsString;
	fn bmrsCVar_get_description(this: *mut ()) -> BmrsString;
	fn bmrsCVar_get_default_value(this: *mut ()) -> BmrsString;
	fn bmrsCVar_has_minimum(this: *mut ()) -> bool;
	fn bmrsCVar_has_maximum(this: *mut ()) -> bool;
	fn bmrsCVar_get_minimum(this: *mut ()) -> f32;
	fn bmrsCVar_get_maximum(this: *mut ()) -> f32;
	fn bmrsCVar_is_hidden(this: *mut ()) -> bool;
	fn bmrsCVar_should_save_to_cfg(this: *mut ()) -> bool;
	fn bmrsCVar_reset_to_default(this: *mut ());
	fn bmrsCVar_notify(this: *mut ());
	fn bmrsCVar_set_string_value(this: *mut (), value: *const BmrsString);
	fn bmrsCVar_set_int_value(this: *mut (), value: i32);
	fn bmrsCVar_set_float_value(this: *mut (), value: f32);
	fn bmrsCVar_set_color_value(this: *mut (), value: LinearColor);
	fn bmrsCVar_add_on_value_changed(this: *mut (), handler: extern "C" fn(old_value: BmrsString, this: *mut (), aux: *mut ()), aux: *mut ());
	fn bmrsCVar_remove_on_value_changed(this: *mut ());
	fn bmrsCVar_is_null(this: *mut ()) -> bool;
	fn bmrsCVar_drop(this: *mut ());
}
