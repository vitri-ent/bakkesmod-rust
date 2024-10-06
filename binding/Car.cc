#include <bakkesmod/wrappers/GameObject/CarWrapper.h>

#include "./Car.hh"
#include "./Common.hh"
#include "./Pri.hh"

extern "C" {
	bmrsCar *bmrsCar_wrap(uintptr_t addr) {
		return (bmrsCar *)new CarWrapper(addr);
	}

	bmrsString bmrsCar_get_owner_name(const bmrsCar *self) {
		CarWrapper *native = (CarWrapper *)self;
		return bmrs::ConvertString(native->GetOwnerName());
	}

	void bmrsCar_demolish(const bmrsCar *self) {
		CarWrapper *native = (CarWrapper *)self;
		native->Demolish();
	}

	bool bmrsCar_has_flip(const bmrsCar *self) {
		CarWrapper *native = (CarWrapper *)self;
		return native->HasFlip() != 0;
	}

	bmrsPri *bmrsCar_get_pri(const bmrsCar *self) {
		CarWrapper *native = (CarWrapper *)self;
		return bmrs::ConvertPri(native->GetPRI());
	}

	void bmrsCar_drop(const bmrsCar *self) {
		CarWrapper *native = (CarWrapper *)self;
		delete native;
	}

	BMRS_ARRAY_IMPL(Car)
}
