#![allow(dead_code)]
use rand_distr::num_traits::Pow;

use super::Material;
use crate::math::dot;
use crate::math::norm;
use crate::math::rand;
use crate::math::refract;
use crate::math::vec3::reflect;
use crate::rendering::{HitRecord, Ray};
use crate::Color;

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

	pub fn reflectance(cosine: f64, ref_index: f64) -> f64 {
		// Schlick's approximation for reflectance.
		// yields the probability of reflectance
		let mut r0 = (1.0 - ref_index) / (1.0 + ref_index);
		r0 *= r0;
		r0 + (1.0 - r0) * (1.0 - cosine).pow(5)
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
		let cos_theta = dot(&(-unit_dir), rec.normal()).min(1.0);
		let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
		// if the ray can be reflected - look up total internal reflection, critical angle
		let direction = match refraction_ratio * sin_theta > 1.0
			|| Dielectric::reflectance(cos_theta, refraction_ratio) > rand()
		{
			true => reflect(&unit_dir, rec.normal()),
			false => refract(&unit_dir, rec.normal(), refraction_ratio),
		};

		Some((
			Color::from(1.0, 1.0, 1.0),
			Ray::from(rec.point(), &direction),
		))
	}
}
