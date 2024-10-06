#include <bakkesmod/wrappers/GameEvent/ServerWrapper.h>
#include <bakkesmod/wrappers/GameObject/BallWrapper.h>

#include "./Actor.hh"
#include "./Car.hh"
#include "./Common.hh"
#include "./Pri.hh"
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

	bmrsArrCar bmrsServer_get_cars(const bmrsServer *self) {
		ServerWrapper *native = (ServerWrapper *)self;
		return bmrs::ConvertArray(native->GetCars());
	}
	
	bmrsArrPri bmrsServer_get_pris(const bmrsServer *self) {
		ServerWrapper *native = (ServerWrapper *)self;
		return bmrs::ConvertArray(native->GetPRIs());
	}

	void bmrsServer_drop(const bmrsServer *self) {
		ServerWrapper *native = (ServerWrapper *)self;
		delete native;
	}
}
