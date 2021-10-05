use crate::{
	color::Color,
	math::Vec3,
	rendering::{HitRecord, Ray},
};

use super::Material;

pub struct Lambertian {
	albedo: Color,
}

impl Lambertian {
	pub fn new() -> Self {
		Self {
			albedo: Color::from(0.0, 0.0, 0.0),
		}
	}

	pub fn from(albedo: Color) -> Self {
		Self { albedo }
	}
}

impl Material for Lambertian {
	fn scatter(&self, _r_in: &Ray, rec: HitRecord) -> Option<(Color, Ray)> {
		// from the book but `target` is unnesessary. You can just use the hemisphere vector

		// let target = rec.point() + Vec3::random_in_hemisphere(rec.normal());
		// let child_ray = Ray::from(rec.point(), &(target - rec.point()));
		// return 0.5 * ray_color(&child_ray, entities, depth - 1);

		// @author -> from the book, but inaccuarate

		// let scatter_direction = rec.point() + Vec3::random_in_hemisphere(rec.normal());
		// let mut scatter_direction = rec.normal() + Vec3::random_unit_vec();

		// if scatter_direction.near_zero() {
		// 	scatter_direction = *rec.normal();
		// }

		// Some((
		// 	self.albedo.clone(),
		// 	Ray::from(rec.point(), &scatter_direction),
		// ))

		// my solution
		Some((
			self.albedo.clone(),
			Ray::from(rec.point(), &Vec3::random_in_hemisphere(rec.normal())),
		))
	}
}
