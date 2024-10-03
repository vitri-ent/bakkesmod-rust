macro_rules! impl_object {
	($name: ident) => {
		impl $crate::wrappers::ObjectT for $name {
			fn new(ptr: *mut ()) -> Self {
				Self(ptr)
			}

			fn try_new(ptr: *mut ()) -> Option<Self> {
				if ptr.is_null() { None } else { Some(Self(ptr)) }
			}

			fn ptr(&self) -> *mut () {
				self.0
			}
		}

		impl $crate::wrappers::UnrealPointer for $name {
			fn from_ptr(addr: *mut ()) -> Self {
				Self(addr)
			}
		}
	};
}

macro_rules! impl_unreal_pointer_struct {
	($name: ident) => {
		impl $crate::wrappers::UnrealPointer for $name {
			fn from_ptr(addr: *mut ()) -> Self {
				unsafe { *(addr as *const Self) }
			}
		}
	};
}

macro_rules! struct_default_new {
	($name: ident) => {
		impl $name {
			pub fn new() -> Self {
				Self(0)
			}
		}
	};
}
