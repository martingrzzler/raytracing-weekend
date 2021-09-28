#![allow(dead_code)]

use crate::math::{dot, Point3};
use crate::rendering::{Hit, HitRecord, Ray};

pub struct Sphere {
	center: Point3,
	radius: f64,
}

impl Sphere {
	pub fn new() -> Self {
		Self {
			center: Point3::new(),
			radius: 0.0,
		}
	}

	pub fn from(c: Point3, r: f64) -> Self {
		Self {
			center: c,
			radius: r,
		}
	}
}

impl Hit for Sphere {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let oc = r.origin() - &self.center;
		let a = r.direction().len_squared();
		let half_b = dot(&oc, &r.direction());
		let c = oc.len_squared() - self.radius * self.radius;

		let discriminant = half_b * half_b - a * c;
		if discriminant < 0.0 {
			return None;
		}

		let d_sqrt = discriminant.sqrt();
		let root = (-half_b - d_sqrt) / a;
		if root < t_min || t_max < root {
			let root = (-half_b + d_sqrt) / a;
			if root < t_min || t_max < root {
				return None;
			}
		}
		let mut rec = HitRecord::new();
		*rec.t_mut() = root;
		*rec.point_mut() = r.at(rec.t());
		let outward_normal = (rec.point() - &self.center) / self.radius;
		rec.calc_normal(&r, outward_normal);
		Some(rec)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::math::{norm, Vec3};
	#[test]
	fn test_should_return_some_for_hit() {
		// ray shooting down the negative z-axis
		let r = Ray::from(&Point3::new(), &Vec3::from(0.0, 0.0, -1.0));
		// sphere sitting away from origin by -1 on z-axis
		let sphere = Sphere::from(Point3::from(0.0, 0.0, -1.0), 0.5);

		assert_eq!(norm(&Vec3::from(0.0, 0.0, -1.0)).len(), 1.0);
		let hit = sphere.hit(&r, 0.0, 5.0);
		assert!(hit.is_some())
	}

	#[test]
	fn test_should_not_hit() {
		let r = Ray::from(&Point3::new(), &norm(&Vec3::from(-0.6, 0.7, -1.0)));
		let sphere = Sphere::from(Point3::from(0.0, 0.0, -1.0), 0.5);
		let hit = sphere.hit(&r, 0.0, 5.0);

		assert!(hit.is_none());
	}
}
