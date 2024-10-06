#pragma once

#include <bakkesmod/wrappers/GameObject/CarComponent/BoostWrapper.h>

struct bmrsBoost;

namespace bmrs {
	static inline bmrsBoost *ConvertBoost(BoostWrapper native) {
		if (!native) {
			return nullptr;
		}
		BoostWrapper *s = new BoostWrapper(native);
		return (bmrsBoost *)s;
	}
}
