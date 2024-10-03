#pragma once

#include <bakkesmod/wrappers/GameEvent/ServerWrapper.h>

struct bmrsServer;

namespace bmrs {
	static inline bmrsServer *ConvertServer(ServerWrapper native) {
		if (!native) {
			return nullptr;
		}
		ServerWrapper *s = new ServerWrapper(native);
		return (bmrsServer *)s;
	}
}
