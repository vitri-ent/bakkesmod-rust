#![allow(unused)]

use std::convert::From;
use std::ffi::{CStr, CString};
use std::fmt;
use std::marker::{PhantomData, Sized};
use std::ops;
use std::os::raw::c_char;

use crate::prelude::*;

#[macro_use]
mod macros;

pub mod actor;
pub mod cvar;
pub mod cvar_manager;
pub mod game;
pub mod server;
pub mod structs;

pub trait UnrealPointer {
	fn from_ptr(addr: *mut ()) -> Self;
}

pub trait ObjectT: UnrealPointer {
	fn new(addr: *mut ()) -> Self;
	fn try_new(addr: *mut ()) -> Option<Self>
	where
		Self: Sized;
	fn ptr(&self) -> *mut ();
}

pub unsafe trait Wrapper: ObjectT {
	unsafe fn wrap(addr: usize) -> Self;
}

pub struct Object(*mut ());
impl_object!(Object);
