use super::{
	actor::ActorT,
	structs::{BmrsString, Vector},
	vehicle::VehicleT,
	ObjectT, Pri, Wrapper
};

#[repr(transparent)]
pub struct Car(*mut ());
impl_object!(Car);

impl ActorT for Car {}
impl VehicleT for Car {}
impl CarT for Car {}

unsafe impl Wrapper for Car {
	unsafe fn wrap_ptr(addr: usize) -> *mut () {
		unsafe { bmrsCar_wrap(addr) }
	}
}

pub trait CarT: VehicleT {
	fn owner_name(&self) -> String {
		unsafe { bmrsCar_get_owner_name(self.ptr()) }.to_string()
	}

	fn demolish(&self) {
		unsafe { bmrsCar_demolish(self.ptr()) };
	}

	fn has_flip(&self) -> bool {
		unsafe { bmrsCar_has_flip(self.ptr()) }
	}

	fn pri(&self) -> Option<Pri> {
		Pri::try_new(unsafe { bmrsCar_get_pri(self.ptr()) })
	}
}

impl Drop for Car {
	fn drop(&mut self) {
		unsafe { bmrsCar_drop(self.0) };
	}
}

extern "C" {
	fn bmrsCar_wrap(addr: usize) -> *mut ();
	fn bmrsCar_get_owner_name(this: *mut ()) -> BmrsString;
	fn bmrsCar_demolish(this: *mut ());
	fn bmrsCar_has_flip(this: *mut ()) -> bool;
	fn bmrsCar_get_pri(this: *mut ()) -> *mut ();
	fn bmrsCar_drop(this: *mut ());
}
