#include <stdint.h>

#include <bakkesmod/wrappers/Engine/UnrealStringWrapper.h>
#include <bakkesmod/wrappers/GameObject/TeamInfoWrapper.h>

#include "./Common.hh"
#include "./TeamInfo.hh"

extern "C" {
	bmrsTeamInfo *bmrsTeamInfo_wrap(uintptr_t addr) {
		return (bmrsTeamInfo *)new TeamInfoWrapper(addr);
	}

	bmrsString bmrsTeamInfo_get_name(const bmrsTeamInfo *self) {
		TeamInfoWrapper *native = (TeamInfoWrapper *)self;
		return bmrs::ConvertString(native->GetTeamName().ToString());
	}

	int32_t bmrsTeamInfo_get_index(const bmrsTeamInfo *self) {
		TeamInfoWrapper *native = (TeamInfoWrapper *)self;
		return native->GetTeamIndex();
	}

	int32_t bmrsTeamInfo_get_score(const bmrsTeamInfo *self) {
		TeamInfoWrapper *native = (TeamInfoWrapper *)self;
		return native->GetScore();
	}

	void bmrsTeamInfo_drop(const bmrsTeamInfo *self) {
		TeamInfoWrapper *native = (TeamInfoWrapper *)self;
		delete native;
	}
}
