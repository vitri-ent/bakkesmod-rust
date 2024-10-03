#include <bakkesmod/wrappers/GameEvent/ServerWrapper.h>
#include <bakkesmod/wrappers/GameObject/BallWrapper.h>

#include "./Actor.hh"
#include "./Common.hh"
#include "./Server.hh"

extern "C" {
	bmrsActor *bmrsServer_get_ball(const bmrsServer *self) {
		ServerWrapper *native = (ServerWrapper *)self;
		return bmrs::ConvertActor(native->GetBall());
	}

	void bmrsServer_spawn_car(const bmrsServer *self, bmrsString *name) {
		ServerWrapper *native = (ServerWrapper *)self;
		native->SpawnCar(23, bmrs::ConvertString(name));
	}

	void bmrsServer_drop(const bmrsServer *self) {
		ServerWrapper *native = (ServerWrapper *)self;
		delete native;
	}
}
