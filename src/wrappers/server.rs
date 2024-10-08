use super::{
	actor::Actor,
	structs::{BmrsArray, BmrsString},
	Car, ObjectT, Pri, UnrealPointer
};

#[repr(transparent)]
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

	fn spawn_bot(&self, name: &str) {
		unsafe { bmrsServer_spawn_bot(self.ptr(), &name.into()) };
	}

	fn cars(&self) -> Vec<Car> {
		unsafe { bmrsServer_get_cars(self.ptr()) }.into_vec(bmrsArrCar_drop)
	}

	fn pris(&self) -> Vec<Pri> {
		unsafe { bmrsServer_get_pris(self.ptr()) }.into_vec(bmrsArrPri_drop)
	}
}

impl Drop for Server {
	fn drop(&mut self) {
		unsafe { bmrsServer_drop(self.0) };
	}
}

extern "C" {
	fn bmrsServer_get_ball(this: *mut ()) -> *mut ();
	fn bmrsServer_spawn_car(this: *mut (), name: *const BmrsString) -> *mut ();
	fn bmrsServer_spawn_bot(this: *mut (), name: *const BmrsString) -> *mut ();
	fn bmrsServer_get_cars(this: *mut ()) -> BmrsArray<Car>;
	fn bmrsServer_get_pris(this: *mut ()) -> BmrsArray<Pri>;
	fn bmrsServer_drop(this: *mut ());

	fn bmrsArrCar_drop(a: *const BmrsArray<Car>);
	fn bmrsArrPri_drop(a: *const BmrsArray<Pri>);
}
