#include <bakkesmod/wrappers/GameObject/CarComponent/BoostWrapper.h>

#include "./Boost.hh"

extern "C" {
	bmrsBoost *bmrsBoost_wrap(uintptr_t addr) {
		return (bmrsBoost *)new BoostWrapper(addr);
	}

	float bmrsBoost_get_percent_full(const bmrsBoost *self) {
		BoostWrapper *native = (BoostWrapper *)self;
		return native->GetPercentBoostFull();
	}
	
	float bmrsBoost_get_amount(const bmrsBoost *self) {
		BoostWrapper *native = (BoostWrapper *)self;
		return native->GetCurrentBoostAmount();
	}

	void bmrsBoost_drop(const bmrsBoost *self) {
		BoostWrapper *native = (BoostWrapper *)self;
		delete native;
	}
}
