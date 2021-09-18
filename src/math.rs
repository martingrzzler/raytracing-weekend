use core::panic;
use std::ops;

#[derive(Debug, PartialEq)]
pub struct Vec3 {
	e: (f64, f64, f64),
}

#[allow(dead_code)]
pub type Point3 = Vec3;

#[allow(dead_code)]
pub type Color = Vec3;

#[allow(dead_code)]
impl Vec3 {
	pub fn new() -> Self {
		Self { e: (0.0, 0.0, 0.0) }
	}

	pub fn from(x: f64, y: f64, z: f64) -> Self {
		Self { e: (x, y, z) }
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

impl ops::AddAssign for Vec3 {
	fn add_assign(&mut self, rhs: Vec3) {
		*self = Self {
			e: (self.e.0 + rhs.e.0, self.e.1 + rhs.e.1, self.e.2 + rhs.e.2),
		}
	}
}

impl ops::Add for Vec3 {
	type Output = Vec3;

	fn add(self, rhs: Vec3) -> Vec3 {
		Self {
			e: (self.e.0 + rhs.e.0, self.e.1 + rhs.e.1, self.e.2 + rhs.e.2),
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
			e: (self.e.0 * rhs.e.0, self.e.1 * rhs.e.1, self.e.2 * self.e.2),
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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_add_two_vec3() {
		let res = Vec3::from(1.0, 2.0, 3.0) + Vec3::from(3.0, 2.0, 1.0);
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
}
