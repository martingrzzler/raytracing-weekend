#![allow(non_upper_case_globals)]
use std::io::{self, Write};

use color::{write_color, Color};
use rayon::prelude::*;
use rendering::ray_color;

use crate::output::pixels_to_file;
use crate::rendering::random_scene;
use crate::{
	camera::Camera,
	math::{rand, Point3, Vec3},
};

mod camera;
mod color;
mod math;
mod output;
mod rendering;

pub fn run(args: Vec<String>) {
	// Image
	const aspect_ratio: f64 = 16.0 / 9.0;
	const image_width: i32 = 100;
	const image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
	let samples_per_pixel = 1;
	let max_depth = 50;

	// Entities
	let entities = random_scene();

	// Camera
	let look_from = Point3::from(13.0, 2.0, 3.0);
	let look_at = Point3::from(0.0, 0.0, 0.0);
	let vup = Vec3::from(0.0, 1.0, 0.0);
	let focus_distance = 10.0;
	let aperture = 0.1;
	let cam = Camera::new(
		look_from,
		look_at,
		vup,
		20.0,
		aspect_ratio,
		aperture,
		focus_distance,
	);

	// Render
	let mut pixels = vec![];
	for j in 0..image_height {
		for i in 0..image_width {
			eprint!(
				"\rProgress: {:.2}%",
				((j * image_width + i + 1) as f64 / ((image_height * image_width) as f64) * 100.0)
			);
			io::stderr().flush().unwrap();
			let pixel_color: Color = (0..samples_per_pixel)
				.into_par_iter()
				.map(|_sample| {
					let u = (i as f64 + rand()) / ((image_width as f64) - 1.0);
					let v = (j as f64 + rand()) / ((image_height as f64) - 1.0);
					let r = cam.calc_ray(u, v);
					ray_color(&r, &entities, max_depth)
				})
				.sum();
			write_color(&mut pixels, pixel_color, samples_per_pixel);
		}
	}

	eprint!("\rWriting to file...");
	pixels_to_file(&pixels, image_height, image_width, args);
	eprintln!("\nDone.");
	io::stderr().flush().unwrap();
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_run_should_run() {
		run(vec!["".to_string(), "test.ppm".to_string()])
	}
}
