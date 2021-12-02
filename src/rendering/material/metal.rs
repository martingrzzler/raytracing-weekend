#![allow(dead_code)]
use crate::{
	math::vec3::{dot, reflect, Vec3},
	rendering::{HitRecord, Ray},
	Color,
};

use super::Material;

pub struct Metal {
	albedo: Color,
	fuzz: f64,
}

impl Metal {
	pub fn new() -> Self {
		Self {
			albedo: Color::from(0.0, 0.0, 0.0),
			fuzz: 0.0,
		}
	}

	pub fn from(albedo: Color, fuzz: f64) -> Self {
		assert!(fuzz <= 1.0);
		Self { albedo, fuzz }
	}
}

impl Material for Metal {
	fn scatter(&self, r_in: &Ray, rec: HitRecord) -> Option<(Color, Ray)> {
		let reflected = reflect(r_in.direction(), rec.normal());
		let scattered = Ray::from(
			rec.point(),
			&(reflected + self.fuzz * Vec3::random_in_unit_sphere()),
		);
		// ommited the check because this can't happen I think
		if dot(scattered.direction(), rec.normal()) > 0.0 {
			return Some((self.albedo.clone(), scattered));
		}

		None
	}
}
