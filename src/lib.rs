use std::io::{self, Write};

use color::write_color;
use math::Vec3;
use rendering::{ray_color, Ray, Sphere};

use crate::{math::Point3, rendering::Hit};

mod color;
mod math;
mod rendering;

pub fn run() {
	// Image
	let aspect_ratio = 16.0 / 9.0;
	let image_width = 400;
	let image_height = (image_width as f64 / aspect_ratio) as i32;

	// Entities
	let mut entities: Vec<Box<dyn Hit>> = vec![];
	entities.push(Box::new(Sphere::from(Point3::from(0.0, 0.0, -1.0), 0.5)));
	entities.push(Box::new(Sphere::from(
		Point3::from(0.0, -20.5, -1.0),
		100.0,
	)));

	// Camera
	let viewport_height = 2.0;
	let viewport_width = aspect_ratio * viewport_height;
	let focal_length = 1.0;

	let origin = Point3::new();
	let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
	let vertical = Vec3::from(0.0, viewport_height, 0.0);

	let upper_left_corner =
		&origin - &horizontal / 2.0 + &vertical / 2.0 - Vec3::from(0.0, 0.0, focal_length);

	// Render
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
			let pixel_color = ray_color(&r, &entities);
			write_color(pixel_color);
		}
	}
	eprintln!("\nDone.");
	io::stderr().flush().unwrap();
}
