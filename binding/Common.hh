#pragma once

#include <string>

#include <bakkesmod/wrappers/wrapperstructs.h>

extern "C" {
	struct bmrsString {
		const char *ptr;
		size_t len;
		std::string *backed;
	};

	struct bmrsLinearColor {
		float r;
		float g;
		float b;
		float a;
	};
}

namespace bmrs {
	static inline bmrsString ConvertString(std::string string) {
		std::string *s = new std::string(string);
		return bmrsString {
			.ptr = s->c_str(),
			.len = s->length(),
			.backed = s
		};
	}

	static inline std::string ConvertString(bmrsString *string) {
		return std::string(string->ptr, string->len);
	}
	static inline bmrsLinearColor ConvertColor(LinearColor color) {
		return bmrsLinearColor {
			.r = color.R,
			.g = color.G,
			.b = color.B,
			.a = color.A
		};
	}
}
