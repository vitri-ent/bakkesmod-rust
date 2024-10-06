use super::{
	actor::ActorT,
	structs::{BmrsString, Vector},
	team_info::TeamInfo,
	vehicle::VehicleT,
	Car, ObjectT, PlayerReplicationInfoT, UnrealPointer, Wrapper
};

#[repr(transparent)]
pub struct Pri(*mut ());
impl_object!(Pri);

impl ActorT for Pri {}
impl PlayerReplicationInfoT for Pri {}
impl PriT for Pri {}

unsafe impl Wrapper for Pri {
	unsafe fn wrap_ptr(addr: usize) -> *mut () {
		unsafe { bmrsPri_wrap(addr) }
	}
}

pub trait PriT: ActorT {
	fn car(&self) -> Car {
		Car::new(unsafe { bmrsPri_get_car(self.ptr()) })
	}

	fn index(&self) -> usize {
		(unsafe { bmrsPri_get_spectator_shortcut(self.ptr()) }) as _
	}
}

impl Drop for Pri {
	fn drop(&mut self) {
		unsafe { bmrsPri_drop(self.0) };
	}
}

extern "C" {
	fn bmrsPri_wrap(addr: usize) -> *mut ();
	fn bmrsPri_get_car(this: *mut ()) -> *mut ();
	fn bmrsPri_get_spectator_shortcut(this: *mut ()) -> i32;
	fn bmrsPri_drop(this: *mut ());
}
