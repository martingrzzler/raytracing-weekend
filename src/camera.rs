use crate::{
	math::{Point3, Vec3},
	rendering::Ray,
};

pub struct Camera {
	origin: Point3,
	upper_left_corner: Point3,
	horizontal: Vec3,
	vertical: Vec3,
}

impl Camera {
	pub fn new() -> Self {
		let aspect_ratio = 16.0 / 9.0;
		let viewport_height = 2.0;
		let viewport_width = aspect_ratio * viewport_height;
		let focal_length = 1.0;

		let origin = Point3::new();
		let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
		let vertical = Vec3::from(0.0, viewport_height, 0.0);
		let upper_left_corner =
			&origin - &horizontal / 2.0 + &vertical / 2.0 - Vec3::from(0.0, 0.0, focal_length);

		Self {
			origin,
			horizontal,
			vertical,
			upper_left_corner,
		}
	}

	pub fn calc_ray(&self, u: f64, v: f64) -> Ray {
		let ray_dir =
			&self.upper_left_corner + &self.horizontal * u - &self.vertical * v - &self.origin;

		Ray::from(&self.origin, &ray_dir)
	}
}
