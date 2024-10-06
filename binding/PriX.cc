#include <memory>
#include <stdint.h>

#include <bakkesmod/wrappers/GameObject/PriXWrapper.h>

#include "./PriX.hh"

extern "C" {
	bmrsPriX *bmrsPriX_wrap(uintptr_t addr) {
		return (bmrsPriX *)new PriXWrapper(addr);
	}

	void bmrsPriX_drop(const bmrsPriX *self) {
		PriXWrapper *native = (PriXWrapper *)self;
		delete native;
	}
}
