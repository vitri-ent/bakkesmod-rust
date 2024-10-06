#pragma once

#include <string>

#include <bakkesmod/wrappers/arraywrapper.h>
#include <bakkesmod/wrappers/WrapperStructs.h>

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

struct bmrsVec3 {
	float x;
	float y;
	float z;
};

struct bmrsQuat {
	float x;
	float y;
	float z;
	float w;
};

#define BMRS_ARRAY(T)                                                                \
	struct T##Wrapper;                                                               \
	struct bmrsArr##T {                                                              \
		bmrs##T **ptr;                                                               \
		size_t len;                                                                  \
		std::vector<bmrs##T *> *backed;                                              \
	};                                                                               \
	namespace bmrs {                                                                 \
		static inline bmrsArr##T ConvertArray(ArrayWrapper<T##Wrapper> array) {      \
			std::vector<bmrs##T *> *v = new std::vector<bmrs##T *>(array.Count());   \
			for (int i = 0; i < array.Count(); i++) {                                \
				v->at(i) = bmrs::Convert##T(array.Get(i));                           \
			}                                                                        \
			return bmrsArr##T {                                                      \
				.ptr = v->data(),                                                    \
				.len = v->size(),                                                    \
				.backed = v                                                          \
			};                                                                       \
		}                                                                            \
	}
#define BMRS_ARRAY_IMPL(T)                  \
	void bmrsArr##T##_drop(bmrsArr##T *a) { \
		delete a->backed;                   \
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

	static inline bmrsVec3 ConvertVec3(Vector vec) {
		return bmrsVec3 {
			.x = vec.X,
			.y = vec.Y,
			.z = vec.Z
		};
	}

	static inline Vector ConvertVec3(bmrsVec3 vec) {
		return Vector(vec.x, vec.y, vec.z);
	}

	static inline bmrsQuat ConvertQuat(Quat quat) {
		return bmrsQuat {
			.x = quat.X,
			.y = quat.Y,
			.z = quat.Z,
			.w = quat.W
		};
	}

	static inline bmrsQuat ConvertQuat(Rotator rotator) {
		return bmrs::ConvertQuat(RotatorToQuat(rotator));
	}

	static inline Rotator ConvertQuat(bmrsQuat quat) {
		return QuatToRotator(Quat(quat.w, quat.x, quat.y, quat.z));
	}
}
