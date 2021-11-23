#![allow(dead_code)]

use crate::{
	math::{cross, norm, radians, Point3, Vec3},
	rendering::Ray,
};

pub struct CameraProps {
	pub look_from: Point3,
	pub look_at: Point3,
	pub aperture: f64,
	pub focus_distance: f64,
	pub field_of_view: f64,
	pub aspect_ratio: f64,
}

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
	pub fn from(
		CameraProps {
			look_at,
			look_from,
			aperture,
			focus_distance,
			field_of_view,
			aspect_ratio,
		}: CameraProps,
	) -> Self {
		let theta = radians(field_of_view);
		let h = (theta / 2.0).tan();
		// tan of 45 deg is 1
		let viewport_height = 2.0 * h;
		let viewport_width = aspect_ratio * viewport_height;

		let w = norm(&(&look_from - &look_at));
		let u = norm(&cross(&Vec3::from(0.0, 1.0, 0.0), &w));
		let v = cross(&w, &u);

		let origin = look_from;
		let horizontal = focus_distance * viewport_width * u;
		let vertical = focus_distance * viewport_height * v;
		let upper_left_corner = &origin - &horizontal / 2.0 + &vertical / 2.0 - &w * focus_distance;
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
