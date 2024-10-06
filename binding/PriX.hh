#pragma once

#include <memory>

#include <bakkesmod/wrappers/GameObject/PriXWrapper.h>

struct bmrsPriX;

namespace bmrs {
	static inline bmrsPriX *ConvertPriX(PriXWrapper native) {
		if (!native) {
			return nullptr;
		}
		PriXWrapper *s = new PriXWrapper(native);
		return (bmrsPriX *)s;
	}
}
