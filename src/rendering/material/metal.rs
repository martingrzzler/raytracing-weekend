#![allow(dead_code)]
use crate::{
	color::Color,
	math::vec3::reflect,
	rendering::{HitRecord, Ray},
};

use super::Material;

pub struct Metal {
	albedo: Color,
}

impl Metal {
	pub fn new() -> Self {
		Self {
			albedo: Color::from(0.0, 0.0, 0.0),
		}
	}

	pub fn from(albedo: Color) -> Self {
		Self { albedo }
	}
}

impl Material for Metal {
	fn scatter(&self, r_in: &Ray, rec: HitRecord) -> Option<(Color, Ray)> {
		let reflected = reflect(r_in.direction(), rec.normal());
		let scattered = Ray::from(rec.point(), &reflected);
		// ommited the check because this can't happen I think
		// if dot(scattered.direction(), rec.normal()) > 0.0 {
		return Some((self.albedo, scattered));
		// }

		// None
	}
}
