#![allow(dead_code)]
use super::Material;
use crate::color::Color;
use crate::math::norm;
use crate::math::refract;
use crate::rendering::{HitRecord, Ray};

pub struct Dielectric {
	refraction_index: f64,
}

impl Dielectric {
	pub fn new() -> Self {
		Self {
			refraction_index: 0.0,
		}
	}

	pub fn from(refraction_index: f64) -> Self {
		Self { refraction_index }
	}
}

impl Material for Dielectric {
	fn scatter(&self, r_in: &Ray, rec: HitRecord) -> Option<(Color, Ray)> {
		let refraction_ratio = if rec.front_face() {
			1.0 / self.refraction_index
		} else {
			self.refraction_index
		};
		let unit_dir = norm(r_in.direction());
		let refracted = refract(&unit_dir, rec.normal(), refraction_ratio);

		Some((
			Color::from(1.0, 1.0, 1.0),
			Ray::from(rec.point(), &refracted),
		))
	}
}
