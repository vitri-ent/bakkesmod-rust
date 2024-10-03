#pragma once

#include <bakkesmod/wrappers/Engine/ActorWrapper.h>

struct bmrsActor;

namespace bmrs {
	static inline bmrsActor *ConvertActor(ActorWrapper native) {
		if (!native) {
			return nullptr;
		}
		ActorWrapper *s = new ActorWrapper(native);
		return (bmrsActor *)s;
	}
}
