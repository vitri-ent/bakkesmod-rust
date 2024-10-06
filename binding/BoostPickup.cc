#include <bakkesmod/wrappers/GameObject/BoostPickupWrapper.h>

#include "./BoostPickup.hh"

extern "C" {
	bmrsBoostPickup *bmrsBoostPickup_wrap(uintptr_t addr) {
		return (bmrsBoostPickup *)new BoostPickupWrapper(addr);
	}

	float bmrsBoostPickup_get_boost_amount(const bmrsBoostPickup *self) {
		BoostPickupWrapper *native = (BoostPickupWrapper *)self;
		return native->GetBoostAmount();
	}

	unsigned char bmrsBoostPickup_get_boost_type(const bmrsBoostPickup *self) {
		BoostPickupWrapper *native = (BoostPickupWrapper *)self;
		return native->GetBoostType();
	}

	void bmrsBoostPickup_drop(const bmrsBoostPickup *self) {
		BoostPickupWrapper *native = (BoostPickupWrapper *)self;
		delete native;
	}
}
