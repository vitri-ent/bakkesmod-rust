#pragma once

#include <bakkesmod/wrappers/cvarwrapper.h>

#include "./Common.hh"

struct bmrsCVar;

typedef void (*bmrsCVarValueChangedHandler)(bmrsString old, const bmrsCVar *newVar, void *);

namespace bmrs {
	static inline bmrsCVar *ConvertCVar(CVarWrapper native) {
		CVarWrapper *s = new CVarWrapper(native);
		return (bmrsCVar *)s;
	}
}
