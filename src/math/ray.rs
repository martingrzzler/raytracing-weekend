#![allow(dead_code)]
use crate::math::{Point3, Vec3};

pub struct Ray {
	orig: Point3,
	dir: Vec3,
}

impl Ray {
	pub fn new() -> Self {
		Self {
			orig: Point3::new(),
			dir: Vec3::new(),
		}
	}

	pub fn from(orig: &Point3, dir: &Vec3) -> Self {
		Self {
			orig: orig.clone(),
			dir: dir.clone(),
		}
	}

	pub fn origin(&self) -> &Point3 {
		&self.orig
	}

	pub fn direction(&self) -> &Vec3 {
		&self.dir
	}

	pub fn at(&self, t: f64) -> Point3 {
		&self.orig + &(&self.dir * t)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_scaled_ray() {
		let ray = Ray::from(&Point3::from(0.0, 0.0, 0.0), &Vec3::from(1.0, 1.0, 1.0));
		assert_eq!(ray.at(3.0), Point3::from(3.0, 3.0, 3.0))
	}
}
