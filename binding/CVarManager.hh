#pragma once

#include <bakkesmod/wrappers/cvarmanagerwrapper.h>

#include "./Common.hh"

struct bmrsCVarManager;

extern "C" {
	typedef void (*bmrsCommandNotifier)(bmrsString *args, size_t n_args, void *aux);
}
