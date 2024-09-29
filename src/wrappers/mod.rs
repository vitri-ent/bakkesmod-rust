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

pub mod canvas;
pub mod cvar;
pub mod mmr;
pub mod structs;
pub mod unreal;

pub trait UnrealPointer {
	fn from_ptr(addr: usize) -> Self;
}

pub trait Object: UnrealPointer {
	fn new(addr: usize) -> Self;
	fn try_new(addr: usize) -> Option<Self>
	where
		Self: Sized;
	fn addr(&self) -> usize;
}

pub struct ObjectWrapper(pub usize);
impl_object!(ObjectWrapper);
