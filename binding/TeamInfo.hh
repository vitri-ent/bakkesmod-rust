#pragma once

#include <bakkesmod/wrappers/GameObject/TeamInfoWrapper.h>

struct bmrsTeamInfo;

namespace bmrs {
	static inline bmrsTeamInfo *ConvertTeamInfo(TeamInfoWrapper native) {
		if (!native) {
			return nullptr;
		}
		TeamInfoWrapper *s = new TeamInfoWrapper(native);
		return (bmrsTeamInfo *)s;
	}
}
