use crate::internal;
use crate::prelude::actor::Actor;
use crate::prelude::game::EventParams;
use crate::prelude::server::Server;

pub fn hook_event<F>(event: &str, cb: F)
where
	F: FnMut(&Actor, EventParams) + 'static
{
	internal::bakkesmod().game.hook_event(event, cb);
}

pub fn is_in_game() -> bool {
	internal::bakkesmod().game.is_in_game()
}

pub fn current_state() -> Option<Server> {
	internal::bakkesmod().game.current_state()
}
