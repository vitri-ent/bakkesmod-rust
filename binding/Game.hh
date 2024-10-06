#pragma once

#include <bakkesmod/wrappers/GameWrapper.h>

#include "./Actor.hh"
#include "./Common.hh"

struct bmrsGame;

typedef void (*bmrsEventCallback)(uintptr_t callerAddr, void *params, bmrsString eventName, void *aux);
