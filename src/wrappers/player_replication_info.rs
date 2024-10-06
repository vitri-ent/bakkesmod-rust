use super::{
	actor::ActorT,
	structs::{BmrsString, Vector},
	team_info::TeamInfo,
	vehicle::VehicleT,
	ObjectT, UnrealPointer, Wrapper
};

#[repr(transparent)]
pub struct PlayerReplicationInfo(*mut ());
impl_object!(PlayerReplicationInfo);

impl ActorT for PlayerReplicationInfo {}

unsafe impl Wrapper for PlayerReplicationInfo {
	unsafe fn wrap_ptr(addr: usize) -> *mut () {
		unsafe { bmrsPlayerReplicationInfo_wrap(addr) }
	}
}

pub trait PlayerReplicationInfoT: ActorT {
	fn player_name(&self) -> String {
		unsafe { bmrsPlayerReplicationInfo_get_name(self.ptr()) }.to_string()
	}

	fn id(&self) -> i32 {
		unsafe { bmrsPlayerReplicationInfo_get_id(self.ptr()) }
	}

	fn is_bot(&self) -> bool {
		unsafe { bmrsPlayerReplicationInfo_is_bot(self.ptr()) }
	}

	fn team(&self) -> TeamInfo {
		TeamInfo::new(unsafe { bmrsPlayerReplicationInfo_get_team(self.ptr()) })
	}
}

impl Drop for PlayerReplicationInfo {
	fn drop(&mut self) {
		unsafe { bmrsPlayerReplicationInfo_drop(self.0) };
	}
}

extern "C" {
	fn bmrsPlayerReplicationInfo_wrap(addr: usize) -> *mut ();
	fn bmrsPlayerReplicationInfo_get_name(this: *mut ()) -> BmrsString;
	fn bmrsPlayerReplicationInfo_get_id(this: *mut ()) -> i32;
	fn bmrsPlayerReplicationInfo_is_bot(this: *mut ()) -> bool;
	fn bmrsPlayerReplicationInfo_get_team(this: *mut ()) -> *mut ();
	fn bmrsPlayerReplicationInfo_drop(this: *mut ());
}
