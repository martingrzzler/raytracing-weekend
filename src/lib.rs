use std::io::{self, Write};

use math::{dot, norm, Vec3};
use rendering::Ray;

use crate::{
	color::write_color,
	math::{Color, Point3},
};

mod color;
mod math;
mod rendering;

pub fn run() {
	let aspect_ratio = 16.0 / 9.0;
	let image_width = 400;
	let image_height = (image_width as f64 / aspect_ratio) as i32;

	let viewport_height = 2.0;
	let viewport_width = aspect_ratio * viewport_height;
	let focal_length = 1.0;

	let origin = Point3::new();
	let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
	let vertical = Vec3::from(0.0, viewport_height, 0.0);

	let upper_left_corner =
		&origin - &horizontal / 2.0 + &vertical / 2.0 - Vec3::from(0.0, 0.0, focal_length);

	print!("P3\n{} {}\n255\n", image_width, image_height);
	for j in 0..image_height {
		eprint!(
			"\rProgress: {}%",
			(((j as f64) + 1.0) / (image_height as f64) * 100.0) as i32
		);
		io::stderr().flush().unwrap();
		for i in 0..image_width {
			let u = (i as f64) / ((image_width as f64) - 1.0);
			let v = (j as f64) / ((image_height as f64) - 1.0);
			let ray_dir = &upper_left_corner + &horizontal * u - &vertical * v - &origin;
			let r = Ray::from(&origin, &ray_dir);
			let pixel_color = ray_color(&r);
			write_color(pixel_color);
		}
	}
	eprintln!("\nDone.");
	io::stderr().flush().unwrap();
}

fn ray_color(r: &Ray) -> Color {
	let sphere_center = Point3::from(0.0, 0.0, -1.0);
	let t = hit_sphere(&sphere_center, 0.5, r);
	if t > 0.0 {
		let normal = norm(&(sphere_center - r.at(t)));
		return 0.5 * Color::from(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
	}
	let unit_dir = norm(r.direction());
	let t = 0.5 * (unit_dir.y() + 1.0);
	Color::from(1.0, 1.0, 1.0) * (1.0 - t) + Color::from(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
	let oc = r.origin() - center;
	let a = r.direction().len_squared();
	let b_half = dot(&oc, r.direction());
	let c = oc.len_squared() - radius * radius;
	let discriminant = b_half * b_half - a * c;
	if discriminant < 0.0 {
		-1.0
	} else {
		(-b_half - discriminant.sqrt()) / a
	}
}
