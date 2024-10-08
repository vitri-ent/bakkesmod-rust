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

mod actor;
pub use self::actor::{Actor, ActorT};
mod boost;
pub use self::boost::{Boost, BoostT};
mod boost_pickup;
pub use self::boost_pickup::{BoostPickup, BoostPickupT};
mod car;
pub use self::car::{Car, CarT};
mod cvar;
pub use self::cvar::{CVar, CVarValue, CVarValueSetter};
mod cvar_manager;
pub use self::cvar_manager::CVarManager;
mod game;
pub use self::game::{EventParams, Game};
mod player_replication_info;
pub use self::player_replication_info::{PlayerReplicationInfo, PlayerReplicationInfoT};
mod pri;
pub use self::pri::{Pri, PriT};
mod server;
pub use self::server::{Server, ServerT};
mod structs;
pub use self::structs::{LinearColor, Quat, Vector, Vector2, VehicleInputs};
mod team_info;
pub use self::team_info::{TeamInfo, TeamInfoT};
mod vehicle;
pub use self::vehicle::{Vehicle, VehicleT};

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
	unsafe fn wrap_ptr(addr: usize) -> *mut ();

	unsafe fn wrap(addr: usize) -> Self
	where
		Self: Sized
	{
		Self::new(unsafe { Self::wrap_ptr(addr) })
	}
}

pub struct Object(*mut ());
impl_object!(Object);
