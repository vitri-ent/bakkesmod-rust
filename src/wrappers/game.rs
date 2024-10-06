use std::{
	collections::HashMap,
	mem::{self, ManuallyDrop},
	ptr,
	sync::Mutex
};

use super::{actor::Actor, car::Car, server::Server, structs::BmrsString, ActorT, ObjectT, UnrealPointer, Wrapper};

pub(crate) struct HookCallback {
	wrapper_factory: unsafe fn(usize) -> *mut (),
	cb: Box<dyn FnMut(&Actor, EventParams)>
}

pub struct Game {
	pub ptr: *mut (),
	pub(crate) hook_cbs: Mutex<HashMap<String, Vec<Box<HookCallback>>>>
}

impl Game {
	pub(crate) fn new(ptr: *mut ()) -> Self {
		Self {
			ptr,
			hook_cbs: Mutex::new(HashMap::new())
		}
	}

	pub fn is_in_game(&self) -> bool {
		unsafe { bmrsGame_is_in_game(self.ptr) }
	}

	pub fn hook_event<F, A: ActorT + Wrapper>(&self, event: &str, cb: F)
	where
		F: FnMut(&A, EventParams) + 'static
	{
		let cb = Box::new(HookCallback {
			wrapper_factory: A::wrap_ptr,
			cb: unsafe {
				mem::transmute::<Box<dyn FnMut(&A, EventParams)>, Box<dyn FnMut(&Actor, EventParams)>>(Box::new(cb) as Box<dyn FnMut(&A, EventParams)>)
			}
		});
		unsafe {
			bmrsGame_hook_event(self.ptr, &event.into(), Self::hook_event_cb, &*cb as *const _ as *mut ());
		};
		self.hook_cbs.lock().unwrap().entry(event.to_string()).or_default().push(cb);
	}

	pub fn current_state(&self) -> Option<Server> {
		Server::try_new(unsafe { bmrsGame_get_current_state(self.ptr) })
	}

	pub fn local_car(&self) -> Option<Car> {
		Car::try_new(unsafe { bmrsGame_get_local_car(self.ptr) })
	}

	extern "C" fn hook_event_cb(caller: usize, params: *mut (), event_name: BmrsString, aux: *mut ()) {
		let mut cb = unsafe { Box::from_raw(aux as *mut HookCallback) };
		let actor = ManuallyDrop::new(Actor::from_ptr(unsafe { (cb.wrapper_factory)(caller) }));
		(cb.cb)(&actor, EventParams::new(params));
		let _ = Box::into_raw(cb);
	}
}

pub struct EventParams {
	inner: *mut u8
}

impl EventParams {
	pub(crate) fn new(inner: *mut ()) -> Self {
		Self { inner: inner.cast() }
	}

	pub unsafe fn read<T>(&self, byte_pos: usize) -> T {
		unsafe { ptr::read(self.inner.add(byte_pos).cast()) }
	}

	pub unsafe fn get<T>(&self, byte_pos: usize) -> &T {
		unsafe { &*self.inner.add(byte_pos).cast::<T>() }
	}

	pub unsafe fn get_mut<T>(&self, byte_pos: usize) -> &mut T {
		unsafe { &mut *self.inner.add(byte_pos).cast::<T>() }
	}

	pub unsafe fn get_actor<T: Wrapper>(&self, byte_pos: usize) -> T {
		let addr = unsafe { self.read::<usize>(byte_pos) };
		unsafe { T::wrap(addr) }
	}
}

extern "C" {
	fn bmrsGame_is_in_game(this: *mut ()) -> bool;
	fn bmrsGame_hook_event(
		this: *mut (),
		event_name: *const BmrsString,
		cb: extern "C" fn(caller: usize, params: *mut (), event_name: BmrsString, aux: *mut ()),
		aux: *mut ()
	);
	fn bmrsGame_hook_event_post(
		this: *mut (),
		event_name: *const BmrsString,
		cb: extern "C" fn(caller: usize, params: *mut (), event_name: BmrsString, aux: *mut ()),
		aux: *mut ()
	);
	fn bmrsGame_get_local_car(this: *mut ()) -> *mut ();
	fn bmrsGame_get_current_state(this: *mut ()) -> *mut ();
}
