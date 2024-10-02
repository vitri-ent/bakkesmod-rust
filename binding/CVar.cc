#include <string>

#include <bakkesmod/wrappers/cvarwrapper.h>

#include "./Common.hh"
#include "./CVar.hh"

extern "C" {
	bmrsString bmrsCVar_get_name(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		return bmrs::ConvertString(native->getCVarName());
	}

	int bmrsCVar_get_int_value(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		return native->getIntValue();
	}
	
	float bmrsCVar_get_float_value(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		return native->getFloatValue();
	}
	
	bool bmrsCVar_get_bool_value(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		return native->getBoolValue();
	}

	bmrsLinearColor bmrsCVar_get_color_value(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		return bmrs::ConvertColor(native->getColorValue());
	}

	bmrsString bmrsCVar_get_string_value(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		return bmrs::ConvertString(native->getStringValue());
	}
	
	bmrsString bmrsCVar_get_description(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		return bmrs::ConvertString(native->getDescription());
	}

	bmrsString bmrsCVar_get_default_value(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		return bmrs::ConvertString(native->GetDefaultValue());
	}

	bool bmrsCVar_has_minimum(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		return native->HasMinimum();
	}
	
	bool bmrsCVar_has_maximum(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		return native->HasMaximum();
	}
	
	float bmrsCVar_get_minimum(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		return native->GetMinimum();
	}
	
	float bmrsCVar_get_maximum(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		return native->GetMaximum();
	}

	bool bmrsCVar_is_hidden(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		return native->IsHidden();
	}
	
	bool bmrsCVar_should_save_to_cfg(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		return native->ShouldSaveToCfg();
	}
	
	void bmrsCVar_reset_to_default(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		native->ResetToDefault();
	}
	
	void bmrsCVar_notify(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		native->notify();
	}

	void bmrsCVar_set_string_value(const bmrsCVar *self, bmrsString *value) {
		CVarWrapper *native = (CVarWrapper *)self;
		native->setValue(bmrs::ConvertString(value));
	}

	void bmrsCVar_set_int_value(const bmrsCVar *self, int32_t value) {
		CVarWrapper *native = (CVarWrapper *)self;
		native->setValue((int)value);
	}

	void bmrsCVar_set_float_value(const bmrsCVar *self, float value) {
		CVarWrapper *native = (CVarWrapper *)self;
		native->setValue(value);
	}

	void bmrsCVar_set_color_value(const bmrsCVar *self, bmrsLinearColor value) {
		CVarWrapper *native = (CVarWrapper *)self;
		native->setValue(LinearColor(value.r, value.g, value.b, value.a));
	}

	void bmrsCVar_add_on_value_changed(const bmrsCVar *self, bmrsCVarValueChangedHandler handler, void *aux) {
		CVarWrapper *native = (CVarWrapper *)self;
		native->addOnValueChanged([=](std::string old, CVarWrapper newWrapper) {
			const bmrsCVar *newVar = bmrs::ConvertCVar(newWrapper);
			handler(bmrs::ConvertString(old), newVar, aux);
		});
	}

	void bmrsCVar_remove_on_value_changed(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		native->removeOnValueChanged();
	}

	bool bmrsCVar_is_null(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		return native->IsNull();
	}

	void bmrsCVar_drop(const bmrsCVar *self) {
		CVarWrapper *native = (CVarWrapper *)self;
		delete native;
	}
};
