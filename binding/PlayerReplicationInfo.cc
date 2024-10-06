#include <memory>
#include <stdint.h>

#include <bakkesmod/wrappers/Engine/UnrealStringWrapper.h>
#include <bakkesmod/wrappers/GameObject/PlayerReplicationInfoWrapper.h>

#include "./Common.hh"
#include "TeamInfo.hh"
#include "./PlayerReplicationInfo.hh"

extern "C" {
	bmrsPlayerReplicationInfo *bmrsPlayerReplicationInfo_wrap(uintptr_t addr) {
		return (bmrsPlayerReplicationInfo *)new PlayerReplicationInfoWrapper(addr);
	}

	bmrsString bmrsPlayerReplicationInfo_get_name(const bmrsPlayerReplicationInfo *self) {
		PlayerReplicationInfoWrapper *native = (PlayerReplicationInfoWrapper *)self;
		return bmrs::ConvertString(native->GetPlayerName().ToString());
	}

	int32_t bmrsPlayerReplicationInfo_get_id(const bmrsPlayerReplicationInfo *self) {
		PlayerReplicationInfoWrapper *native = (PlayerReplicationInfoWrapper *)self;
		return native->GetPlayerID();
	}

	bool bmrsPlayerReplicationInfo_is_bot(const bmrsPlayerReplicationInfo *self) {
		PlayerReplicationInfoWrapper *native = (PlayerReplicationInfoWrapper *)self;
		return native->GetbBot() != 0;
	}

	bmrsTeamInfo *bmrsPlayerReplicationInfo_get_team(const bmrsPlayerReplicationInfo *self) {
		PlayerReplicationInfoWrapper *native = (PlayerReplicationInfoWrapper *)self;
		return bmrs::ConvertTeamInfo(native->GetTeam());
	}

	void bmrsPlayerReplicationInfo_drop(const bmrsPlayerReplicationInfo *self) {
		PlayerReplicationInfoWrapper *native = (PlayerReplicationInfoWrapper *)self;
		delete native;
	}
}
