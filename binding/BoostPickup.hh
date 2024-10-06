#pragma once

#include <bakkesmod/wrappers/GameObject/BoostPickupWrapper.h>

struct bmrsBoostPickup;

namespace bmrs {
	static inline bmrsBoostPickup *ConvertBoostPickup(BoostPickupWrapper native) {
		if (!native) {
			return nullptr;
		}
		BoostPickupWrapper *s = new BoostPickupWrapper(native);
		return (bmrsBoostPickup *)s;
	}
}
