#include <string>

#include <bakkesmod/wrappers/GameWrapper.h>
#include <bakkesmod/wrappers/Engine/ActorWrapper.h>

#include "./Car.hh"
#include "./Common.hh"
#include "./Game.hh"
#include "./Server.hh"

extern "C" {
	bool bmrsGame_is_in_game(const bmrsGame *self) {
		GameWrapper *native = (GameWrapper *)self;
		return native->IsInGame();
	}

	void bmrsGame_hook_event(
		const bmrsGame *self,
		bmrsString *eventName,
		bmrsEventCallback cb,
		void *aux
	) {
		GameWrapper *native = (GameWrapper *)self;
		native->HookEventWithCaller<ActorWrapper>(
			bmrs::ConvertString(eventName),
			[=](ActorWrapper caller, void *params, std::string eventName) {
				cb(caller.memory_address, params, bmrs::ConvertString(eventName), aux);
			}
		);
	}

	void bmrsGame_hook_event_post(
		const bmrsGame *self,
		bmrsString *eventName,
		bmrsEventCallback cb,
		void *aux
	) {
		GameWrapper *native = (GameWrapper *)self;
		native->HookEventWithCallerPost<ActorWrapper>(
			bmrs::ConvertString(eventName),
			[=](ActorWrapper caller, void *params, std::string eventName) {
				cb(caller.memory_address, params, bmrs::ConvertString(eventName), aux);
			}
		);
	}
	
	bmrsServer *bmrsGame_get_current_state(const bmrsGame *self) {
		GameWrapper *native = (GameWrapper *)self;
		return bmrs::ConvertServer(native->GetGameEventAsServer());
	}

	bmrsCar *bmrsGame_get_local_car(const bmrsGame *self) {
		GameWrapper *native = (GameWrapper *)self;
		return bmrs::ConvertCar(native->GetLocalCar());
	}
}
