use super::{
	actor::ActorT,
	structs::{BmrsString, Vector},
	vehicle::VehicleT,
	ObjectT, Wrapper
};

#[repr(transparent)]
pub struct BoostPickup(*mut ());
impl_object!(BoostPickup);

impl ActorT for BoostPickup {}
impl BoostPickupT for BoostPickup {}

unsafe impl Wrapper for BoostPickup {
	unsafe fn wrap_ptr(addr: usize) -> *mut () {
		unsafe { bmrsBoostPickup_wrap(addr) }
	}
}

pub trait BoostPickupT: ActorT {
	fn get_boost_amount(&self) -> f32 {
		unsafe { bmrsBoostPickup_get_boost_amount(self.ptr()) }
	}

	fn get_boost_type(&self) -> u8 {
		unsafe { bmrsBoostPickup_get_boost_type(self.ptr()) }
	}
}

impl Drop for BoostPickup {
	fn drop(&mut self) {
		unsafe { bmrsBoostPickup_drop(self.0) };
	}
}

extern "C" {
	fn bmrsBoostPickup_wrap(addr: usize) -> *mut ();
	fn bmrsBoostPickup_get_boost_amount(this: *mut ()) -> f32;
	fn bmrsBoostPickup_get_boost_type(this: *mut ()) -> u8;
	fn bmrsBoostPickup_drop(this: *mut ());
}
