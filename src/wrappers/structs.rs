use core::{slice, str};
use std::convert::From;
use std::ffi::{CStr, CString};
use std::hash::{Hash, Hasher};
use std::marker::{PhantomData, Sized};
use std::mem::ManuallyDrop;
use std::ops::{self, Deref};
use std::os::raw::c_char;
use std::{fmt, ptr};

use super::*;

#[repr(C)]
pub struct BmrsString {
	ptr: *const u8,
	len: usize,
	backing_ptr: *const ()
}

impl Deref for BmrsString {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		unsafe { std::str::from_utf8_unchecked(slice::from_raw_parts(self.ptr, self.len)) }
	}
}

impl PartialEq for BmrsString {
	fn eq(&self, other: &Self) -> bool {
		self.deref().eq(other.deref())
	}
}
impl Eq for BmrsString {}

impl Hash for BmrsString {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.deref().hash(state)
	}
}

impl Drop for BmrsString {
	fn drop(&mut self) {
		if self.backing_ptr.is_null() {
			// rust-allocated string
			drop(unsafe { String::from_raw_parts(self.ptr.cast_mut(), self.len, self.len) });
		} else {
			// C++-allocated string
			extern "C" {
				fn bmrsString_drop(s: *mut BmrsString);
			};
			unsafe {
				bmrsString_drop(self);
			}
		}
	}
}

impl From<&str> for BmrsString {
	fn from(value: &str) -> Self {
		Self::from(value.to_string())
	}
}

impl From<String> for BmrsString {
	fn from(mut s: String) -> Self {
		let s = ManuallyDrop::new(s.into_boxed_str());
		let (ptr, len) = (s.as_ptr(), s.len());
		BmrsString { ptr, len, backing_ptr: ptr::null() }
	}
}

#[repr(C)]
pub struct BmrsArray<T: Wrapper> {
	ptr: *mut *mut (),
	len: usize,
	backed: *mut PhantomData<T>
}

impl<T: Wrapper> BmrsArray<T> {
	pub fn into_vec(self, dropper: unsafe extern "C" fn(d: *const Self)) -> Vec<T> {
		let mut x = Vec::with_capacity(self.len);
		for i in 0..self.len {
			x.push(unsafe { T::from_ptr(*self.ptr.add(i)) });
		}
		unsafe { dropper(&self) };
		x
	}
}

impl<T: Wrapper> Deref for BmrsArray<T> {
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		unsafe { slice::from_raw_parts(self.ptr.cast::<T>(), self.len) }
	}
}

#[repr(C)]
pub struct RLString {
	data: usize,
	count: u32,
	max: u32
}

impl RLString {
	pub fn new() -> RLString {
		RLString { data: 0, count: 0, max: 0 }
	}
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vector {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Vector {
	pub fn new() -> Vector {
		Vector { x: 0.0, y: 0.0, z: 0.0 }
	}

	pub fn from(x: f32, y: f32, z: f32) -> Vector {
		Vector { x, y, z }
	}
}

impl fmt::Display for Vector {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {}, {})", self.x, self.y, self.z)
	}
}

impl ops::Add for Vector {
	type Output = Vector;

	fn add(self, rhs: Vector) -> Self::Output {
		Vector {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector2 {
	x: i32,
	y: i32
}

impl Vector2 {
	pub fn from(x: i32, y: i32) -> Vector2 {
		Vector2 { x, y }
	}

	pub fn new() -> Vector2 {
		Vector2 { x: 0, y: 0 }
	}
}

impl ops::Add for Vector2 {
	type Output = Vector2;

	fn add(self, rhs: Vector2) -> Self::Output {
		Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector2f {
	x: f32,
	y: f32
}

impl Vector2f {
	pub fn new_from(x: f32, y: f32) -> Vector2f {
		Vector2f { x, y }
	}

	pub fn new() -> Vector2f {
		Vector2f { x: 0.0, y: 0.0 }
	}
}

impl From<Vector2> for Vector2f {
	fn from(vec2: Vector2) -> Self {
		let x: f32 = vec2.x as f32;
		let y: f32 = vec2.y as f32;
		Vector2f { x, y }
	}
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rotator {
	pitch: i32,
	yaw: i32,
	roll: i32
}

impl Rotator {
	pub fn new() -> Rotator {
		Rotator { pitch: 0, yaw: 0, roll: 0 }
	}

	pub fn from(pitch: i32, yaw: i32, roll: i32) -> Rotator {
		Rotator { pitch, yaw, roll }
	}
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Quat {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32
}

impl Quat {
	pub fn new() -> Quat {
		Quat { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
	}

	pub fn from(x: f32, y: f32, z: f32, w: f32) -> Quat {
		Quat { x, y, z, w }
	}
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
	r: u8,
	g: u8,
	b: u8,
	a: u8
}

impl Color {
	pub fn new() -> Color {
		Color { r: 0, g: 0, b: 0, a: 0 }
	}

	pub fn from(r: u8, g: u8, b: u8, a: u8) -> Color {
		Color { r, g, b, a }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LinearColor {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32
}
impl_unreal_pointer_struct!(LinearColor);

impl LinearColor {
	pub fn new() -> Self {
		LinearColor { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
	}

	pub fn from(r: f32, g: f32, b: f32, a: f32) -> Self {
		LinearColor { r, g, b, a }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ReplicatedRBState {
	quat: Quat,
	loc: Vector,
	lin_vel: Vector,
	ang_vel: Vector,
	time: f32,
	b_sleeping: bool,
	b_new_data: bool
}
impl_unreal_pointer_struct!(ReplicatedRBState);

impl ReplicatedRBState {
	pub fn new() -> Self {
		Self {
			quat: Quat::new(),
			loc: Vector::new(),
			lin_vel: Vector::new(),
			ang_vel: Vector::new(),
			time: 0.0,
			b_sleeping: false,
			b_new_data: false
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VehicleInputs {
	pub throttle: f32,
	pub steer: f32,
	pub pitch: f32,
	pub yaw: f32,
	pub roll: f32,
	pub dodge_forward: f32,
	pub dodge_strafe: f32,
	pub flags: u8
}
impl_unreal_pointer_struct!(VehicleInputs);

impl VehicleInputs {
	pub const JUMP: u8 = 1 << 1;
	pub const BOOST: u8 = 1 << 3 | 1 << 2;
	pub const BRAKE: u8 = 1 << 5;

	pub const fn new() -> Self {
		Self {
			throttle: 0.0,
			steer: 0.0,
			pitch: 0.0,
			yaw: 0.0,
			roll: 0.0,
			dodge_forward: 0.0,
			dodge_strafe: 0.0,
			flags: 0
		}
	}

	pub const fn has(&self, flag: u8) -> bool {
		self.flags & flag == flag
	}

	pub const fn set(&mut self, flag: u8) {
		self.flags |= flag;
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SkillRating {
	mu: f32,
	sigma: f32
}
impl_unreal_pointer_struct!(SkillRating);

impl SkillRating {
	pub fn new() -> Self {
		SkillRating { mu: 0.0, sigma: 0.0 }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SkillRank {
	tier: i32,
	division: i32,
	matches_player: i32
}
impl_unreal_pointer_struct!(SkillRank);

impl SkillRank {
	pub fn new() -> Self {
		SkillRank {
			tier: 0,
			division: 0,
			matches_player: 0
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UniqueNetId {
	id: u64
}
impl_unreal_pointer_struct!(UniqueNetId);

impl UniqueNetId {
	pub fn new() -> Self {
		UniqueNetId { id: 0 }
	}

	pub fn from(id: u64) -> Self {
		UniqueNetId { id }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WheelContactData {
	contact_info: u32, // TODO: use bitfields
	has_contact_change_time: f32,
	actor: usize,
	component: usize,
	location: Vector,
	normal: Vector,
	lat_direction: Vector,
	long_direction: Vector,
	phys_mat_prop: Vector
}

impl_unreal_pointer_struct!(WheelContactData);

impl WheelContactData {
	pub fn new() -> Self {
		WheelContactData {
			contact_info: 0,
			has_contact_change_time: 0.0,
			actor: 0,
			component: 0,
			location: Vector::new(),
			normal: Vector::new(),
			lat_direction: Vector::new(),
			long_direction: Vector::new(),
			phys_mat_prop: Vector::new()
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct POV {
	location: Vector,
	rotation: Rotator,
	fov: f32
}
impl_unreal_pointer_struct!(POV);

impl POV {
	pub fn new() -> Self {
		POV {
			location: Vector::new(),
			rotation: Rotator::new(),
			fov: 0.0
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PredictionInfo {
	time: f32,
	arch_top_time: f32,
	location: Vector,
	velocity: Vector,
	arch_top: Vector,
	arch_top_velocity: Vector,
	hit_info: u32 // TODO: use bitfields
}
impl_unreal_pointer_struct!(PredictionInfo);

impl PredictionInfo {
	pub fn new() -> Self {
		PredictionInfo {
			time: 0.0,
			arch_top_time: 0.0,
			location: Vector::new(),
			velocity: Vector::new(),
			arch_top: Vector::new(),
			arch_top_velocity: Vector::new(),
			hit_info: 0
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TViewTarget {
	target: usize,
	controller: usize,
	pov: POV,
	aspect_ratio: f32,
	pri: usize
}
impl_unreal_pointer_struct!(TViewTarget);

impl TViewTarget {
	pub fn new() -> Self {
		TViewTarget {
			target: 0,
			controller: 0,
			pov: POV::new(),
			aspect_ratio: 0.0,
			pri: 0
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WorldContactData {
	has_contact: u32,
	location: Vector,
	velocity: Vector,
	normal: Vector
}
impl_unreal_pointer_struct!(WorldContactData);

impl WorldContactData {
	pub fn new() -> Self {
		WorldContactData {
			has_contact: 0,
			location: Vector::new(),
			velocity: Vector::new(),
			normal: Vector::new()
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Double(f64);
impl_unreal_pointer_struct!(Double);

impl Double {
	pub fn new() -> Double {
		Double(0.0)
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sample {
	low: f32,
	high: f32
}
impl_unreal_pointer_struct!(Sample);

impl Sample {
	pub fn new() -> Self {
		Sample { low: 0.0, high: 0.0 }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StickyForceData {
	ground: f32,
	wall: f32
}
impl_unreal_pointer_struct!(StickyForceData);

impl StickyForceData {
	pub fn new() -> Self {
		StickyForceData { ground: 0.0, wall: 0.0 }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProfileCameraSettings {
	fov: f32,
	height: f32,
	pitch: f32,
	distane: f32,
	stiffness: f32,
	swivel_speed: f32,
	transition_speed: f32
}
impl_unreal_pointer_struct!(ProfileCameraSettings);

impl ProfileCameraSettings {
	pub fn new() -> Self {
		ProfileCameraSettings {
			fov: 0.0,
			height: 0.0,
			pitch: 0.0,
			distane: 0.0,
			stiffness: 0.0,
			swivel_speed: 0.0,
			transition_speed: 0.0
		}
	}
}
