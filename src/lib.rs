use std::io::{self, Write};
use std::rc::Rc;

use color::{write_color, Color};
use rendering::{ray_color, Sphere};

use crate::rendering::Dielectric;
use crate::{
	camera::Camera,
	math::{rand, Point3},
	rendering::{Hit, Lambertian, Metal},
};

mod camera;
mod color;
mod math;
mod rendering;

pub fn run() {
	// Image
	let aspect_ratio = 16.0 / 9.0;
	let image_width = 400;
	let image_height = (image_width as f64 / aspect_ratio) as i32;
	let samples_per_pixel = 100;
	let max_depth = 50;

	// Materials
	let grass = Rc::new(Lambertian::from(Color::from(0.8, 0.8, 0.0)));
	let glass = Rc::new(Dielectric::from(1.5));
	let smooth_metal = Rc::new(Metal::from(Color::from(0.8, 0.8, 0.8), 0.3));
	let rough_metal = Rc::new(Metal::from(Color::from(0.8, 0.6, 0.2), 1.0));

	// Entities
	let mut entities: Vec<Box<dyn Hit>> = vec![];
	entities.push(Box::new(Sphere::from(
		Point3::from(0.0, 0.0, -1.0),
		0.5,
		glass.clone(),
	)));
	entities.push(Box::new(Sphere::from(
		Point3::from(1.0, 0.0, -1.0),
		0.5,
		rough_metal.clone(),
	)));
	entities.push(Box::new(Sphere::from(
		Point3::from(-1.0, 0.0, -1.0),
		0.5,
		smooth_metal.clone(),
	)));
	entities.push(Box::new(Sphere::from(
		Point3::from(0.0, -100.5, -1.0),
		100.0,
		grass.clone(),
	)));

	// Camera
	let cam = Camera::new();

	// Render
	print!("P3\n{} {}\n255\n", image_width, image_height);
	for j in 0..image_height {
		eprint!(
			"\rProgress: {}%",
			(((j as f64) + 1.0) / (image_height as f64) * 100.0) as i32
		);
		io::stderr().flush().unwrap();
		for i in 0..image_width {
			let mut pixel_color = Color::new();
			for _ in 0..samples_per_pixel {
				let u = (i as f64 + rand()) / ((image_width as f64) - 1.0);
				let v = (j as f64 + rand()) / ((image_height as f64) - 1.0);
				let r = cam.calc_ray(u, v);
				pixel_color += ray_color(&r, &entities, max_depth);
			}
			write_color(pixel_color, samples_per_pixel);
		}
	}
	eprintln!("\nDone.");
	io::stderr().flush().unwrap();
}
