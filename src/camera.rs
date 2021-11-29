#![allow(dead_code)]

use crate::{
	math::{cross, norm, radians, Point3, Vec3},
	rendering::Ray,
};

pub struct CameraParams {
	pub look_from: Point3,
	pub look_at: Point3,
	pub field_of_view: f64,
	pub aspect_ratio: f64,
}

pub struct Camera {
	origin: Point3,
	viewport_width: f64,
	viewport_height: f64,
	w: Vec3,
	u: Vec3,
	v: Vec3,
}

impl Camera {
	pub fn from(
		CameraParams {
			look_at,
			look_from,
			field_of_view,
			aspect_ratio,
		}: CameraParams,
	) -> Self {
		let (viewport_width, viewport_height) =
			Camera::calc_viewport_dimensions(field_of_view, aspect_ratio);
		let (u, v, w) = Camera::calc_cam_coordinate_system(look_from, look_at);
		Self {
			viewport_height,
			viewport_width,
			origin: look_from,
			u,
			v,
			w,
		}
	}

	pub fn viewport_width(&self) -> f64 {
		self.viewport_width
	}
	pub fn viewport_height(&self) -> f64 {
		self.viewport_height
	}
	pub fn origin(&self) -> &Vec3 {
		&self.origin
	}
	pub fn coord_system(&self) -> (&Vec3, &Vec3, &Vec3) {
		(&self.u, &self.v, &self.w)
	}

	fn calc_viewport_dimensions(field_of_view: f64, aspect_ratio: f64) -> (f64, f64) {
		let theta = radians(field_of_view);
		let h = (theta / 2.0).tan();
		let viewport_height = 2.0 * h;
		let viewport_width = aspect_ratio * viewport_height;

		(viewport_width, viewport_height)
	}

	fn calc_cam_coordinate_system(look_from: Point3, look_at: Point3) -> (Vec3, Vec3, Vec3) {
		let w = norm(&(&look_from - &look_at));
		let u = norm(&cross(&Vec3::from(0.0, 1.0, 0.0), &w));
		let v = cross(&w, &u);

		(u, v, w)
	}
}
pub struct DefocusBlurGenerator {
	upper_left_corner: Point3,
	horizontal: Vec3,
	vertical: Vec3,
	cam: Camera,
	lens_radius: f64,
}

impl DefocusBlurGenerator {
	pub fn from(cam: Camera, aperture: f64, focus_distance: f64) -> Self {
		let (u, v, w) = cam.coord_system();
		let horizontal = focus_distance * cam.viewport_width() * u;
		let vertical = focus_distance * cam.viewport_height() * v;
		Self {
			upper_left_corner: cam.origin() - &horizontal / 2.0 + &vertical / 2.0 - w * focus_distance,
			vertical,
			horizontal,
			lens_radius: aperture / 2.0,
			cam,
		}
	}
}

impl RayGenerator for DefocusBlurGenerator {
	fn gen_ray(&self, s: f64, t: f64) -> Ray {
		let (u, v, _) = self.cam.coord_system();
		let rd = self.lens_radius * Vec3::random_in_unit_disk();
		let offset = u * rd.x() + v * rd.y();
		let ray_dir = &self.upper_left_corner + &self.horizontal * s
			- &self.vertical * t
			- self.cam.origin()
			- &offset;

		Ray::from(&(self.cam.origin() + &offset), &ray_dir)
	}
}

pub struct PlainGenerator {
	cam: Camera,
	upper_left_corner: Point3,
	horizontal: Vec3,
	vertical: Vec3,
}

impl PlainGenerator {
	pub fn from(cam: Camera) -> Self {
		let (u, v, w) = cam.coord_system();
		let horizontal = cam.viewport_width() * u;
		let vertical = cam.viewport_height() * v;
		Self {
			upper_left_corner: cam.origin() - &horizontal / 2.0 + &vertical / 2.0 - w,
			vertical,
			horizontal,
			cam,
		}
	}
}

impl RayGenerator for PlainGenerator {
	fn gen_ray(&self, s: f64, t: f64) -> Ray {
		let ray_dir =
			&self.upper_left_corner + &self.horizontal * s - &self.vertical * t - self.cam.origin();

		Ray::from(self.cam.origin(), &ray_dir)
	}
}

pub trait RayGenerator {
	fn gen_ray(&self, s: f64, t: f64) -> Ray;
}

// 	// key here to understand is that rays are created within the lens_radius but because the offset is subtracted
// 	// from the ray direction, the ray hits always the same point on the plane focus_distance away.
// 	// Objects being hit by the ray before or after the plane will appear blurred consequently.
