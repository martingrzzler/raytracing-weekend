#![allow(dead_code)]

use std::rc::Rc;

use crate::math::{dot, Point3, Ray, Vec3};
use crate::scene::{Hit, HitRecord, Lambertian, Material};

pub struct Sphere {
	center: Point3,
	radius: f64,
	mat: Rc<dyn Material>,
}

impl Sphere {
	pub fn new() -> Self {
		Self {
			center: Point3::new(),
			radius: 0.0,
			mat: Rc::new(Lambertian::new()),
		}
	}

	pub fn from(c: Point3, r: f64, mat: Rc<dyn Material>) -> Self {
		Self {
			center: c,
			radius: r,
			mat,
		}
	}

	fn solve_quadratic(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<f64> {
		let oc = r.origin() - &self.center;
		let a = r.direction().len_squared();
		let half_b = dot(&oc, &r.direction());
		let c = oc.len_squared() - (self.radius * self.radius);

		let discriminant = (half_b * half_b) - (a * c);
		if discriminant < 0.0 {
			return None;
		}

		let d_sqrt = discriminant.sqrt();
		let mut root = (-half_b - d_sqrt) / a;
		if root < t_min || t_max < root {
			root = (-half_b + d_sqrt) / a;
			if root < t_min || t_max < root {
				return None;
			}
		}

		Some(root)
	}

	fn outward_normal(&self, hit_p: &Point3) -> Vec3 {
		(hit_p - &self.center) / self.radius
	}
}

impl Hit for Sphere {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		if let Some(root) = self.solve_quadratic(r, t_min, t_max) {
			let mut rec = HitRecord::new(Rc::clone(&self.mat));
			*rec.t_mut() = root;
			*rec.point_mut() = r.at(rec.t());
			rec.calc_normal(&r, self.outward_normal(rec.point()));
			Some(rec)
		} else {
			None
		}
	}
}

unsafe impl Send for Sphere {}
unsafe impl Sync for Sphere {}

#[cfg(test)]
mod test {
	use super::*;
	use crate::math::{norm, Vec3};
	#[test]
	fn test_should_return_some_for_hit() {
		// ray shooting down the negative z-axis
		let r = Ray::from(&Point3::new(), &Vec3::from(0.0, 0.0, -1.0));
		let mat = Lambertian::new();
		// sphere sitting away from origin by -1 on z-axis
		let sphere = Sphere::from(Point3::from(0.0, 0.0, -1.0), 0.5, Rc::new(mat));

		assert_eq!(norm(&Vec3::from(0.0, 0.0, -1.0)).len(), 1.0);
		let hit = sphere.hit(&r, 0.0, 5.0);
		assert!(hit.is_some())
	}

	#[test]
	fn test_should_not_hit() {
		let r = Ray::from(&Point3::new(), &norm(&Vec3::from(-0.6, 0.7, -1.0)));
		let mat = Lambertian::new();
		let sphere = Sphere::from(Point3::from(0.0, 0.0, -1.0), 0.5, Rc::new(mat));
		let hit = sphere.hit(&r, 0.0, 5.0);

		assert!(hit.is_none());
	}

	#[test]
	fn test_outward_normal() {
		let r = Ray::from(&Point3::new(), &norm(&Vec3::from(0.0, 0.0, -1.0)));
		let mat = Lambertian::new();
		let sphere = Sphere::from(Point3::from(0.0, 0.0, -1.0), 0.5, Rc::new(mat));

		let outward_normal = sphere.outward_normal(&Point3::from(0.0, 0.0, -0.5));

		assert_eq!(Vec3::from(0.0, 0.0, 1.0), outward_normal)
	}
}
