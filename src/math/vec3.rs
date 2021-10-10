#![allow(dead_code)]
use super::{rand, rand_rng};
use core::panic;
use std::ops::{self};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
	pub e: (f64, f64, f64),
}

pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
	a.e.0 * b.e.0 + a.e.1 * b.e.1 + a.e.2 * b.e.2
}

pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
	Vec3 {
		e: (
			a.e.1 * b.e.2 - a.e.2 * b.e.1,
			a.e.2 * b.e.0 - a.e.0 * b.e.2,
			a.e.0 * b.e.1 - a.e.1 * b.e.0,
		),
	}
}

pub fn norm(v: &Vec3) -> Vec3 {
	v / v.len()
}

pub fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
	v - 2.0 * dot(v, normal) * normal
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
	let cos_theta = dot(&(-uv), n).min(1.0);
	let r_out_perp = etai_over_etat * (uv + cos_theta * n);
	let r_out_parallel = -((1.0 - r_out_perp.len_squared()).abs().sqrt()) * n;
	r_out_perp + r_out_parallel
}

pub type Point3 = Vec3;

impl Vec3 {
	pub fn new() -> Self {
		Self { e: (0.0, 0.0, 0.0) }
	}

	pub fn from(x: f64, y: f64, z: f64) -> Self {
		Self { e: (x, y, z) }
	}

	pub fn from_rand() -> Self {
		Self {
			e: (rand(), rand(), rand()),
		}
	}

	pub fn from_rand_rng(min: f64, max: f64) -> Self {
		Self {
			e: (rand_rng(min, max), rand_rng(min, max), rand_rng(min, max)),
		}
	}

	pub fn near_zero(&self) -> bool {
		let s = 1e-8;
		self.e.0.abs() < s && self.e.1.abs() < s && self.e.2 < s
	}

	pub fn random_in_unit_sphere() -> Self {
		loop {
			let p = Vec3::from_rand_rng(-1.0, 1.0);
			if p.len_squared() >= 1.0 {
				continue;
			}
			return p;
		}
	}

	pub fn random_unit_vec() -> Self {
		norm(&Vec3::random_in_unit_sphere())
	}

	pub fn random_in_hemisphere(normal: &Self) -> Self {
		let res = Vec3::random_in_unit_sphere();
		if dot(&res, normal) > 0.0 {
			res
		} else {
			-res
		}
	}

	pub fn random_in_unit_disk() -> Self {
		loop {
			let p = Vec3::from(rand_rng(-1.0, 1.0), rand_rng(-1.0, 1.0), 0.0);
			if p.len_squared() >= 1.0 {
				continue;
			}

			return p;
		}
	}

	pub fn x(&self) -> f64 {
		self.e.0
	}

	pub fn y(&self) -> f64 {
		self.e.1
	}

	pub fn z(&self) -> f64 {
		self.e.2
	}

	pub fn round_to(&self, to: i32) -> Self {
		Self {
			e: (
				(to as f64 * self.e.0).round() / to as f64,
				(to as f64 * self.e.1).round() / to as f64,
				(to as f64 * self.e.2).round() / to as f64,
			),
		}
	}

	pub fn len(&self) -> f64 {
		self.len_squared().sqrt()
	}

	pub fn len_squared(&self) -> f64 {
		self.e.0 * self.e.0 + self.e.1 * self.e.1 + self.e.2 * self.e.2
	}
}

impl ops::Neg for Vec3 {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self {
			e: (-self.e.0, -self.e.1, -self.e.2),
		}
	}
}

impl ops::Neg for &Vec3 {
	type Output = Vec3;

	fn neg(self) -> Self::Output {
		Vec3 {
			e: (-self.e.0, -self.e.1, -self.e.2),
		}
	}
}

impl ops::AddAssign for Vec3 {
	fn add_assign(&mut self, rhs: Vec3) {
		*self = Self {
			e: (self.e.0 + rhs.e.0, self.e.1 + rhs.e.1, self.e.2 + rhs.e.2),
		}
	}
}

impl ops::Add for &Vec3 {
	type Output = Vec3;

	fn add(self, rhs: &Vec3) -> Vec3 {
		Vec3 {
			e: (self.e.0 + rhs.e.0, self.e.1 + rhs.e.1, self.e.2 + rhs.e.2),
		}
	}
}

impl ops::Add<f64> for &Vec3 {
	type Output = Vec3;

	fn add(self, rhs: f64) -> Vec3 {
		Vec3 {
			e: (self.e.0 + rhs, self.e.1 + rhs, self.e.2 + rhs),
		}
	}
}

impl ops::Add<Vec3> for &Vec3 {
	type Output = Vec3;

	fn add(self, rhs: Vec3) -> Vec3 {
		Vec3 {
			e: (self.e.0 + rhs.e.0, self.e.1 + rhs.e.1, self.e.2 + rhs.e.2),
		}
	}
}
impl ops::Add for Vec3 {
	type Output = Vec3;

	fn add(self, rhs: Vec3) -> Vec3 {
		Vec3 {
			e: (self.e.0 + rhs.e.0, self.e.1 + rhs.e.1, self.e.2 + rhs.e.2),
		}
	}
}
impl ops::Add<&Vec3> for Vec3 {
	type Output = Vec3;

	fn add(self, rhs: &Vec3) -> Vec3 {
		Vec3 {
			e: (self.e.0 + rhs.e.0, self.e.1 + rhs.e.1, self.e.2 + rhs.e.2),
		}
	}
}

impl ops::SubAssign for Vec3 {
	fn sub_assign(&mut self, rhs: Vec3) {
		*self = Self {
			e: (self.e.0 - rhs.e.0, self.e.1 - rhs.e.1, self.e.2 - rhs.e.2),
		}
	}
}

impl ops::Sub for &Vec3 {
	type Output = Vec3;
	fn sub(self, rhs: &Vec3) -> Self::Output {
		Vec3 {
			e: (self.e.0 - rhs.e.0, self.e.1 - rhs.e.1, self.e.2 - rhs.e.2),
		}
	}
}

impl ops::Sub<Vec3> for &Vec3 {
	type Output = Vec3;
	fn sub(self, rhs: Vec3) -> Self::Output {
		Vec3 {
			e: (self.e.0 - rhs.e.0, self.e.1 - rhs.e.1, self.e.2 - rhs.e.2),
		}
	}
}

impl ops::Sub for Vec3 {
	type Output = Vec3;
	fn sub(self, rhs: Vec3) -> Self::Output {
		Vec3 {
			e: (self.e.0 - rhs.e.0, self.e.1 - rhs.e.1, self.e.2 - rhs.e.2),
		}
	}
}
impl ops::Sub<&Vec3> for Vec3 {
	type Output = Vec3;
	fn sub(self, rhs: &Vec3) -> Self::Output {
		Vec3 {
			e: (self.e.0 - rhs.e.0, self.e.1 - rhs.e.1, self.e.2 - rhs.e.2),
		}
	}
}

impl ops::Index<usize> for Vec3 {
	type Output = f64;

	fn index(&self, i: usize) -> &Self::Output {
		match i {
			0 => &self.e.0,
			1 => &self.e.1,
			2 => &self.e.2,
			_ => panic!("index out of bounds"),
		}
	}
}

impl ops::IndexMut<usize> for Vec3 {
	fn index_mut(&mut self, i: usize) -> &mut Self::Output {
		match i {
			0 => &mut self.e.0,
			1 => &mut self.e.1,
			2 => &mut self.e.2,
			_ => panic!("index out of bounds"),
		}
	}
}

impl ops::MulAssign for Vec3 {
	fn mul_assign(&mut self, rhs: Self) {
		*self = Self {
			e: (self.e.0 * rhs.e.0, self.e.1 * rhs.e.1, self.e.2 * self.e.2),
		}
	}
}

impl ops::Mul for Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: Self) -> Self::Output {
		Self {
			e: (self.e.0 * rhs.e.0, self.e.1 * rhs.e.1, self.e.2 * rhs.e.2),
		}
	}
}

impl ops::Div for Vec3 {
	type Output = Vec3;

	fn div(self, rhs: Vec3) -> Self::Output {
		Self {
			e: (self.e.0 / rhs.e.0, self.e.1 / rhs.e.1, self.e.2 / self.e.2),
		}
	}
}

impl ops::DivAssign for Vec3 {
	fn div_assign(&mut self, rhs: Self) {
		*self = Self {
			e: (self.e.0 / rhs.e.0, self.e.1 / rhs.e.1, self.e.2 / self.e.2),
		}
	}
}

impl ops::Div<f64> for &Vec3 {
	type Output = Vec3;
	fn div(self, rhs: f64) -> Self::Output {
		Vec3 {
			e: (self.e.0 / rhs, self.e.1 / rhs, self.e.2 / rhs),
		}
	}
}
impl ops::Div<f64> for Vec3 {
	type Output = Vec3;
	fn div(self, rhs: f64) -> Self::Output {
		Vec3 {
			e: (self.e.0 / rhs, self.e.1 / rhs, self.e.2 / rhs),
		}
	}
}

impl ops::Mul<f64> for &Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: f64) -> Self::Output {
		Vec3 {
			e: (self.e.0 * rhs, self.e.1 * rhs, self.e.2 * rhs),
		}
	}
}

impl ops::Mul<Vec3> for f64 {
	type Output = Vec3;

	fn mul(self, rhs: Vec3) -> Self::Output {
		Vec3 {
			e: (rhs.e.0 * self, rhs.e.1 * self, rhs.e.2 * self),
		}
	}
}

impl ops::Mul<&Vec3> for f64 {
	type Output = Vec3;

	fn mul(self, rhs: &Vec3) -> Self::Output {
		Vec3 {
			e: (rhs.e.0 * self, rhs.e.1 * self, rhs.e.2 * self),
		}
	}
}

impl ops::Mul<f64> for Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: f64) -> Self::Output {
		Vec3 {
			e: (self.e.0 * rhs, self.e.1 * rhs, self.e.2 * rhs),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_add_two_vec3() {
		let res = &Vec3::from(1.0, 2.0, 3.0) + &Vec3::from(3.0, 2.0, 1.0);
		assert_eq!(res, Vec3::from(4.0, 4.0, 4.0));
	}

	#[test]
	fn test_add_assign() {
		let mut v = Vec3::new();
		v += Vec3::from(1.0, 2.0, 3.0);
		assert_eq!(v, Vec3::from(1.0, 2.0, 3.0));
	}
	#[test]
	fn test_neg() {
		let v = -Vec3::from(1.0, 2.0, 3.0);
		assert_eq!(v, Vec3::from(-1.0, -2.0, -3.0))
	}

	#[test]
	fn test_index() {
		let v = Vec3::from(1.0, 2.0, 3.0);
		assert_eq!(v[0], 1.0);
		assert_eq!(v[1], 2.0);
		assert_eq!(v[2], 3.0);
	}

	#[test]
	#[should_panic]
	fn test_index_out_of_bounds() {
		let v = Vec3::from(1.0, 2.0, 3.0);
		let _ = v[4];
	}

	#[test]
	fn test_index_mut() {
		let mut v = Vec3::from(1.0, 2.0, 3.0);
		v[0] = 3.3;
		assert_eq!(v[0], 3.3);
	}

	#[test]
	fn test_mul_assign() {
		let mut v = Vec3::from(1.0, 2.0, 3.0);
		v *= Vec3::from(1.0, 2.0, 3.0);
		assert_eq!(v, Vec3::from(1.0, 4.0, 9.0));
	}

	#[test]
	fn test_mul() {
		let res = Vec3::from(1.0, 2.0, 3.0) * Vec3::from(1.0, 2.0, 3.0);
		assert_eq!(res, Vec3::from(1.0, 4.0, 9.0));
	}

	#[test]
	fn test_div() {
		let res = Vec3::from(1.0, 2.0, 3.0) / Vec3::from(1.0, 2.0, 3.0);
		assert_eq!(res, Vec3::from(1.0, 1.0, 1.0));
	}

	#[test]
	fn test_div_assign() {
		let mut v = Vec3::from(1.0, 2.0, 3.0);
		v /= Vec3::from(1.0, 4.0, 3.0);
		assert_eq!(v, Vec3::from(1.0, 0.5, 1.0));
	}

	#[test]
	fn test_len_squared() {
		let v = Vec3::from(1.0, 2.0, 3.0);
		assert_eq!(v.len_squared(), 14.0);
	}

	#[test]
	fn test_len() {
		let v = Vec3::from(3.0, 4.0, 0.0);
		assert_eq!(v.len(), 5.0);
	}

	#[test]
	fn test_sub() {
		let res = Vec3::from(1.0, -10.0, 5.0) - Vec3::from(3.0, -20.0, 3.0);
		assert_eq!(res, Vec3::from(-2.0, 10.0, 2.0));
	}

	#[test]
	fn test_sub_assign() {
		let mut v = Vec3::from(1.0, -4.5, 3.2);
		v -= Vec3::from(1.0, 1.0, 1.0);
		assert_eq!(v, Vec3::from(0.0, -5.5, 2.2));
	}

	#[test]
	fn test_dot() {
		let a = Vec3::from(1.0, 2.0, 3.0);
		let b = Vec3::from(-2.0, 4.0, 5.0);
		let dot_product = dot(&a, &b);
		assert_eq!(dot_product, 21.0);
	}

	#[test]
	fn test_cross() {
		let a = Vec3::from(1.0, 2.0, 3.0);
		let b = Vec3::from(-2.0, 4.0, 5.0);
		let cross_vec = cross(&a, &b);
		assert_eq!(cross_vec, Vec3::from(-2.0, -11.0, 8.0));
	}

	#[test]
	fn test_norm() {
		let v = Vec3::from(-2.0, 4.0, 5.0);
		let unit_vec = norm(&v);
		assert_eq!(unit_vec, &v / (45.0 as f64).sqrt());
		assert_eq!(unit_vec.len(), 1.0);
	}

	#[test]
	fn test_div_by_scaler() {
		let v = Vec3::from(-2.0, 4.0, 5.0) / 2.0;
		assert_eq!(v, Vec3::from(-1.0, 2.0, 2.5));
	}

	#[test]
	fn test_mul_with_scaler() {
		let v = Vec3::from(-2.0, 4.0, 5.0) * (1.0 / 2.0);
		assert_eq!(v, Vec3::from(-1.0, 2.0, 2.5));
	}

	#[test]
	fn test_reflect() {
		let normal = Vec3::from(0.0, 1.0, 0.0);
		let v = Vec3::from(-1.0, 1.0, 0.0);
		let r = reflect(&v, &normal);
		assert_eq!(r, Vec3::from(-1.0, -1.0, 0.0));
	}

	#[test]
	fn test_refract() {
		let normal = Vec3::from(0.0, 1.0, 0.0);
		let uv = norm(&Vec3::from(1.0, -1.0, 0.0));
		let etai_over_etat = 1.0 / 1.5;

		assert_eq!(
			refract(&uv, &normal, etai_over_etat).round_to(10_000),
			Vec3::from(0.4714, -0.8819, 0.0)
		);
	}

	#[test]
	fn test_round_to() {
		let v = Vec3::from(1.234, 3.234, 6.34534);
		assert_eq!(v.round_to(100), Vec3::from(1.23, 3.23, 6.35));
	}
}
