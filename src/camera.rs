use crate::{
	math::{cross, norm, radians, Point3, Vec3},
	rendering::Ray,
};

pub struct Camera {
	origin: Point3,
	upper_left_corner: Point3,
	horizontal: Vec3,
	vertical: Vec3,
}

impl Camera {
	pub fn new(look_from: Point3, look_at: Point3, vup: Vec3, v_fov: f64, aspect_ratio: f64) -> Self {
		let theta = radians(v_fov);
		let h = (theta / 2.0).tan();
		// tan of 45 deg is 1
		let viewport_height = 2.0 * h;
		let viewport_width = aspect_ratio * viewport_height;

		let w = norm(&(&look_from - &look_at));
		let u = norm(&cross(&vup, &w));
		let v = cross(&w, &u);

		let origin = look_from;
		let horizontal = viewport_width * u;
		let vertical = viewport_height * v;
		let upper_left_corner = &origin - &horizontal / 2.0 + &vertical / 2.0 - &w;

		Self {
			origin,
			horizontal,
			vertical,
			upper_left_corner,
		}
	}

	pub fn calc_ray(&self, s: f64, t: f64) -> Ray {
		let ray_dir =
			&self.upper_left_corner + &self.horizontal * s - &self.vertical * t - &self.origin;

		Ray::from(&self.origin, &ray_dir)
	}
}
