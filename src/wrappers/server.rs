use super::{actor::Actor, structs::BmrsString, ObjectT, UnrealPointer};

pub struct Server(*mut ());
impl_object!(Server);

impl ServerT for Server {}

pub trait ServerT: ObjectT {
	fn get_ball(&self) -> Option<Actor> {
		Actor::try_new(unsafe { bmrsServer_get_ball(self.ptr()) })
	}

	fn spawn_car(&self, name: &str) {
		unsafe { bmrsServer_spawn_car(self.ptr(), &name.into()) };
	}
}

impl Drop for Server {
	fn drop(&mut self) {
		// unsafe { bmrsServer_drop(self.0) };
	}
}

extern "C" {
	fn bmrsServer_get_ball(this: *mut ()) -> *mut ();
	fn bmrsServer_spawn_car(this: *mut (), name: *const BmrsString) -> *mut ();
	fn bmrsServer_drop(this: *mut ());
}
