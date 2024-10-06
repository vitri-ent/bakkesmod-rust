#include <memory>
#include <stdint.h>

#include <bakkesmod/wrappers/GameObject/PriWrapper.h>

#include "./Car.hh"
#include "./Common.hh"
#include "./Pri.hh"

extern "C" {
	bmrsPri *bmrsPri_wrap(uintptr_t addr) {
		return (bmrsPri *)new PriWrapper(addr);
	}

	bmrsCar *bmrsPri_get_car(const bmrsPri *self) {
		PriWrapper *native = (PriWrapper *)self;
		return bmrs::ConvertCar(native->GetCar());
	}

	int32_t bmrsPri_get_spectator_shortcut(const bmrsPri *self) {
		PriWrapper *native = (PriWrapper *)self;
		return native->GetSpectatorShortcut();
	}

	void bmrsPri_drop(const bmrsPri *self) {
		PriWrapper *native = (PriWrapper *)self;
		delete native;
	}

	BMRS_ARRAY_IMPL(Pri)
}
