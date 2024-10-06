#pragma once

#include <memory>

#include <bakkesmod/wrappers/GameObject/PlayerReplicationInfoWrapper.h>

struct bmrsPlayerReplicationInfo;

namespace bmrs {
	static inline bmrsPlayerReplicationInfo *ConvertPlayerReplicationInfo(PlayerReplicationInfoWrapper native) {
		if (!native) {
			return nullptr;
		}
		PlayerReplicationInfoWrapper *s = new PlayerReplicationInfoWrapper(native);
		return (bmrsPlayerReplicationInfo *)s;
	}
}
