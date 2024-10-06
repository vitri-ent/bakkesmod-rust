#pragma once

#include <bakkesmod/wrappers/GameObject/CarWrapper.h>

#include "./Common.hh"

struct bmrsCar;

namespace bmrs {
	static inline bmrsCar *ConvertCar(CarWrapper native) {
		if (!native) {
			return nullptr;
		}
		CarWrapper *s = new CarWrapper(native);
		return (bmrsCar *)s;
	}
}

BMRS_ARRAY(Car)
