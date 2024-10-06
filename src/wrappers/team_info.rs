use super::{
	actor::ActorT,
	structs::{BmrsString, Vector},
	vehicle::VehicleT,
	ObjectT, Wrapper
};

#[repr(transparent)]
pub struct TeamInfo(*mut ());
impl_object!(TeamInfo);

impl ActorT for TeamInfo {}

unsafe impl Wrapper for TeamInfo {
	unsafe fn wrap_ptr(addr: usize) -> *mut () {
		unsafe { bmrsTeamInfo_wrap(addr) }
	}
}

pub trait TeamInfoT: ActorT {
	fn name(&self) -> String {
		unsafe { bmrsTeamInfo_get_name(self.ptr()) }.to_string()
	}

	fn index(&self) -> u8 {
		(unsafe { bmrsTeamInfo_get_index(self.ptr()) }) as u8
	}

	fn score(&self) -> usize {
		(unsafe { bmrsTeamInfo_get_score(self.ptr()) }) as usize
	}
}

impl Drop for TeamInfo {
	fn drop(&mut self) {
		unsafe { bmrsTeamInfo_drop(self.0) };
	}
}

extern "C" {
	fn bmrsTeamInfo_wrap(addr: usize) -> *mut ();
	fn bmrsTeamInfo_get_name(this: *mut ()) -> BmrsString;
	fn bmrsTeamInfo_get_index(this: *mut ()) -> i32;
	fn bmrsTeamInfo_get_score(this: *mut ()) -> i32;
	fn bmrsTeamInfo_drop(this: *mut ());
}
