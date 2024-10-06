use super::{structs::Vector, ObjectT, Quat, Wrapper};

#[repr(transparent)]
pub struct Actor(*mut ());
impl_object!(Actor);

impl ActorT for Actor {}

unsafe impl Wrapper for Actor {
	unsafe fn wrap_ptr(addr: usize) -> *mut () {
		unsafe { bmrsActor_wrap(addr) }
	}
}

pub trait ActorT: ObjectT {
	fn position(&self) -> Vector {
		unsafe { bmrsActor_get_location(self.ptr()) }
	}

	fn linear_velocity(&self) -> Vector {
		unsafe { bmrsActor_get_linear_velocity(self.ptr()) }
	}

	fn angular_velocity(&self) -> Vector {
		unsafe { bmrsActor_get_angular_velocity(self.ptr()) }
	}

	fn rotation(&self) -> Quat {
		unsafe { bmrsActor_get_rotation(self.ptr()) }
	}

	fn is_hidden(&self) -> bool {
		unsafe { bmrsActor_is_hidden(self.ptr()) }
	}
}

impl Drop for Actor {
	fn drop(&mut self) {
		unsafe { bmrsActor_drop(self.0) };
	}
}

extern "C" {
	fn bmrsActor_wrap(addr: usize) -> *mut ();
	fn bmrsActor_get_location(this: *mut ()) -> Vector;
	fn bmrsActor_get_linear_velocity(this: *mut ()) -> Vector;
	fn bmrsActor_get_angular_velocity(this: *mut ()) -> Vector;
	fn bmrsActor_get_rotation(this: *mut ()) -> Quat;
	fn bmrsActor_is_hidden(this: *mut ()) -> bool;
	fn bmrsActor_drop(this: *mut ());
}
