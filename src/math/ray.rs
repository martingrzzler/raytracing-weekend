#![allow(dead_code)]
use crate::math::{Point3, Vec3};

pub struct Ray {
	orig: Point3,
	dir: Vec3,
}

impl Ray {
	fn new() -> Self {
		Self {
			orig: Point3::new(),
			dir: Vec3::new(),
		}
	}

	fn from(orig: Point3, dir: Vec3) -> Self {
		Self { orig, dir }
	}

	fn origin(&self) -> &Point3 {
		&self.orig
	}

	fn direction(&self) -> &Vec3 {
		&self.dir
	}

	fn at(&self, t: f64) -> Point3 {
		&self.orig + &(&self.dir * t)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_scaled_ray() {
		let ray = Ray::from(Point3::from(0.0, 0.0, 0.0), Vec3::from(1.0, 1.0, 1.0));
		assert_eq!(ray.at(3.0), Point3::from(3.0, 3.0, 3.0))
	}
}
