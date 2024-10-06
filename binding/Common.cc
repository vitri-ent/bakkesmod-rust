#include "./Common.hh"

extern "C" {
	void bmrsString_drop(bmrsString *s) {
		delete s->backed;
	}
}
