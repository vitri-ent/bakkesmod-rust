#pragma once

#include <memory>

#include <bakkesmod/wrappers/GameObject/PriWrapper.h>

#include "./Common.hh"

struct bmrsPri;

namespace bmrs {
	static inline bmrsPri *ConvertPri(PriWrapper native) {
		if (!native) {
			return nullptr;
		}
		PriWrapper *s = new PriWrapper(native);
		return (bmrsPri *)s;
	}
}

BMRS_ARRAY(Pri)
