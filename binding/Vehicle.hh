#pragma once

#include <bakkesmod/wrappers/GameObject/VehicleWrapper.h>

struct bmrsVehicle;

namespace bmrs {
	static inline bmrsVehicle *ConvertVehicle(VehicleWrapper native) {
		if (!native) {
			return nullptr;
		}
		VehicleWrapper *s = new VehicleWrapper(native);
		return (bmrsVehicle *)s;
	}
}
