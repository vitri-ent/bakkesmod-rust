use super::{
	actor::ActorT,
	structs::{BmrsString, Vector},
	vehicle::VehicleT,
	ObjectT, Wrapper
};

#[repr(transparent)]
pub struct Boost(*mut ());
impl_object!(Boost);

impl ActorT for Boost {}
impl BoostT for Boost {}

unsafe impl Wrapper for Boost {
	unsafe fn wrap_ptr(addr: usize) -> *mut () {
		unsafe { bmrsBoost_wrap(addr) }
	}
}

pub trait BoostT: ActorT {
	fn amount(&self) -> f32 {
		unsafe { bmrsBoost_get_percent_full(self.ptr()) }
	}

	fn amount_abs(&self) -> f32 {
		unsafe { bmrsBoost_get_amount(self.ptr()) }
	}
}

impl Drop for Boost {
	fn drop(&mut self) {
		unsafe { bmrsBoost_drop(self.0) };
	}
}

extern "C" {
	fn bmrsBoost_wrap(addr: usize) -> *mut ();
	fn bmrsBoost_get_percent_full(this: *mut ()) -> f32;
	fn bmrsBoost_get_amount(this: *mut ()) -> f32;
	fn bmrsBoost_drop(this: *mut ());
}
