#![allow(dead_code)]

use crate::{
	math::{cross, norm, radians, Point3, Vec3},
	rendering::Ray,
};

pub struct Camera {
	origin: Point3,
	upper_left_corner: Point3,
	horizontal: Vec3,
	vertical: Vec3,
	w: Vec3,
	u: Vec3,
	v: Vec3,
	lens_radius: f64,
}

impl Camera {
	pub fn new(
		look_from: Point3,
		look_at: Point3,
		vup: Vec3,
		v_fov: f64,
		aspect_ratio: f64,
		aperture: f64,
		focus_dist: f64,
	) -> Self {
		let theta = radians(v_fov);
		let h = (theta / 2.0).tan();
		// tan of 45 deg is 1
		let viewport_height = 2.0 * h;
		let viewport_width = aspect_ratio * viewport_height;

		let w = norm(&(&look_from - &look_at));
		let u = norm(&cross(&vup, &w));
		let v = cross(&w, &u);

		let origin = look_from;
		let horizontal = focus_dist * viewport_width * u;
		let vertical = focus_dist * viewport_height * v;
		let upper_left_corner = &origin - &horizontal / 2.0 + &vertical / 2.0 - &w * focus_dist;
		let lens_radius = aperture / 2.0;

		Self {
			origin,
			horizontal,
			vertical,
			upper_left_corner,
			w,
			u,
			v,
			lens_radius,
		}
	}

	pub fn calc_ray(&self, s: f64, t: f64) -> Ray {
		// key here to understand is that rays are created within the lens_radius but because the offset is subtracted
		// from the ray direction, the ray hits always the same point on the plane focus_distance away.
		// Objects being hit by the ray before or after the plane will appear blurred consequently.
		let rd = self.lens_radius * Vec3::random_in_unit_disk();
		let offset = self.u * rd.x() + self.v * rd.y();
		let ray_dir =
			&self.upper_left_corner + &self.horizontal * s - &self.vertical * t - &self.origin - &offset;

		Ray::from(&(&self.origin + &offset), &ray_dir)
	}
}
