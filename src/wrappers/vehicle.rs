use super::{actor::ActorT, structs::Vector, Boost, ObjectT, Wrapper};

#[repr(transparent)]
pub struct Vehicle(*mut ());
impl_object!(Vehicle);

impl ActorT for Vehicle {}

unsafe impl Wrapper for Vehicle {
	unsafe fn wrap_ptr(addr: usize) -> *mut () {
		unsafe { bmrsVehicle_wrap(addr) }
	}
}

pub trait VehicleT: ActorT {
	fn can_jump(&self) -> bool {
		unsafe { bmrsVehicle_can_jump(self.ptr()) }
	}

	fn is_on_wall(&self) -> bool {
		unsafe { bmrsVehicle_is_on_wall(self.ptr()) }
	}

	fn is_on_ground(&self) -> bool {
		unsafe { bmrsVehicle_is_on_ground(self.ptr()) }
	}

	fn boost(&self) -> Boost {
		Boost::new(unsafe { bmrsVehicle_get_boost_component(self.ptr()) })
	}
}

impl Drop for Vehicle {
	fn drop(&mut self) {
		unsafe { bmrsVehicle_drop(self.0) };
	}
}

extern "C" {
	fn bmrsVehicle_wrap(addr: usize) -> *mut ();
	fn bmrsVehicle_can_jump(this: *mut ()) -> bool;
	fn bmrsVehicle_is_on_wall(this: *mut ()) -> bool;
	fn bmrsVehicle_is_on_ground(this: *mut ()) -> bool;
	fn bmrsVehicle_get_boost_component(this: *mut ()) -> *mut ();
	fn bmrsVehicle_drop(this: *mut ());
}
