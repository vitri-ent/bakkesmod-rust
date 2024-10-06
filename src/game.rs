use crate::internal;
use crate::prelude::ActorT;
use crate::wrappers::{Car, EventParams, Server, Wrapper};

pub fn hook_event<F, A>(event: &str, cb: F)
where
	F: FnMut(&A, EventParams) + 'static,
	A: ActorT + Wrapper
{
	internal::bakkesmod().game.hook_event(event, cb);
}

pub fn is_in_game() -> bool {
	internal::bakkesmod().game.is_in_game()
}

pub fn current_state() -> Option<Server> {
	internal::bakkesmod().game.current_state()
}

pub fn local_car() -> Option<Car> {
	internal::bakkesmod().game.local_car()
}
