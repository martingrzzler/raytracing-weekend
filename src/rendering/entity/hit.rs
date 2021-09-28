use crate::math::{Point3, Vec3};
use crate::rendering::Ray;

pub struct HitRecord {
	pub p: Point3,
	pub normal: Vec3,
	pub t: f64,
}

impl HitRecord {
	pub fn new() -> Self {
		Self {
			p: Point3::new(),
			t: 0.0,
			normal: Vec3::new(),
		}
	}
}

pub trait Hit {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
