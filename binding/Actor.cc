#include <bakkesmod/wrappers/Engine/ActorWrapper.h>

#include "./Actor.hh"
#include "./Common.hh"

extern "C" {
	bmrsActor *bmrsActor_wrap(uintptr_t addr) {
		return (bmrsActor *)new ActorWrapper(addr);
	}

	bmrsVec3 bmrsActor_get_location(const bmrsActor *self) {
		ActorWrapper *native = (ActorWrapper *)self;
		return bmrs::ConvertVec3(native->GetLocation());
	}

	void bmrsActor_set_location(const bmrsActor *self, bmrsVec3 loc) {
		ActorWrapper *native = (ActorWrapper *)self;
		native->SetLocation(bmrs::ConvertVec3(loc));
	}

	bmrsVec3 bmrsActor_get_linear_velocity(const bmrsActor *self) {
		ActorWrapper *native = (ActorWrapper *)self;
		return bmrs::ConvertVec3(native->GetVelocity());
	}

	void bmrsActor_set_linear_velocity(const bmrsActor *self, bmrsVec3 vel) {
		ActorWrapper *native = (ActorWrapper *)self;
		native->SetVelocity(bmrs::ConvertVec3(vel));
	}

	bmrsVec3 bmrsActor_get_angular_velocity(const bmrsActor *self) {
		ActorWrapper *native = (ActorWrapper *)self;
		return bmrs::ConvertVec3(native->GetAngularVelocity());
	}

	bool bmrsActor_is_hidden(const bmrsActor *self) {
		ActorWrapper *native = (ActorWrapper *)self;
		return native->GetbHidden();
	}
	
	bool bmrsActor_is_null(const bmrsActor *self) {
		ActorWrapper *native = (ActorWrapper *)self;
		return native->IsNull();
	}

	void bmrsActor_drop(const bmrsActor *self) {
		ActorWrapper *native = (ActorWrapper *)self;
		delete native;
	}
}
