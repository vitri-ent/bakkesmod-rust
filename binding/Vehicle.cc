#include <bakkesmod/wrappers/GameObject/VehicleWrapper.h>

#include "./Boost.hh"
#include "./Vehicle.hh"

extern "C" {
	bmrsVehicle *bmrsVehicle_wrap(uintptr_t addr) {
		return (bmrsVehicle *)new VehicleWrapper(addr);
	}

	bool bmrsVehicle_can_jump(const bmrsVehicle *self) {
		VehicleWrapper *native = (VehicleWrapper *)self;
		return native->GetbCanJump() != 0;
	}
	
	bool bmrsVehicle_is_on_wall(const bmrsVehicle *self) {
		VehicleWrapper *native = (VehicleWrapper *)self;
		return native->IsOnWall();
	}
	
	bool bmrsVehicle_is_on_ground(const bmrsVehicle *self) {
		VehicleWrapper *native = (VehicleWrapper *)self;
		return native->IsOnGround();
	}

	bmrsBoost *bmrsVehicle_get_boost_component(const bmrsVehicle *self) {
		VehicleWrapper *native = (VehicleWrapper *)self;
		return bmrs::ConvertBoost(native->GetBoostComponent());
	}

	void bmrsVehicle_drop(const bmrsVehicle *self) {
		VehicleWrapper *native = (VehicleWrapper *)self;
		delete native;
	}
}
