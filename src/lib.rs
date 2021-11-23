#![allow(non_upper_case_globals)]
pub use rendering::random_scene;

use std::io::{self, Write};

use color::{write_color, Color};
use rendering::ray_color;

use crate::camera::CameraProps;
use crate::output::pixels_to_file;
use crate::rendering::Hit;
use crate::utils::{calc_height, Progress};
use crate::{
	camera::Camera,
	math::{rand, Point3},
};

mod camera;
mod color;
mod math;
mod output;
mod rendering;
mod utils;

pub struct Settings {
	pub aspect_ratio: f64,
	pub image_width: i32,
	pub image_height: i32,
	pub samples_per_pixel: i32,
	pub max_depth: i32,
	pub file_name: String,
	pub look_at: Point3,
	pub look_from: Point3,
	pub focus_distance: f64,
	pub aperture: f64,
	pub field_of_view: f64,
}

impl Settings {
	pub fn new() -> Self {
		let aspect_ratio = 16.0 / 9.0;
		let image_width = 500;
		Settings {
			aspect_ratio,
			image_width,
			image_height: calc_height(image_width, aspect_ratio),
			samples_per_pixel: 50,
			max_depth: 50,
			file_name: "default.ppm".to_string(),
			look_from: Point3::from(13.0, 2.0, 3.0),
			look_at: Point3::from(0.0, 0.0, 0.0),
			focus_distance: 10.0,
			aperture: 0.1,
			field_of_view: 20.0,
		}
	}
}

pub fn render_image(
	scene: Vec<Box<dyn Hit>>,
	Settings {
		aspect_ratio,
		image_width,
		image_height,
		samples_per_pixel,
		max_depth,
		file_name,
		look_from,
		look_at,
		aperture,
		focus_distance,
		field_of_view,
	}: Settings,
) {
	let cam = Camera::from(CameraProps {
		look_at,
		look_from,
		aspect_ratio,
		aperture,
		focus_distance,
		field_of_view,
	});

	let mut pixels = vec![];
	for j in 0..image_height {
		for i in 0..image_width {
			Progress {
				curr_height: j,
				curr_width: i,
				total_height: image_height,
				total_width: image_width,
			}
			.print();
			let pixel_color: Color = (0..samples_per_pixel)
				.map(|_sample| {
					let u = (i as f64 + rand()) / ((image_width as f64) - 1.0);
					let v = (j as f64 + rand()) / ((image_height as f64) - 1.0);
					let r = cam.calc_ray(u, v);
					ray_color(&r, &scene, max_depth)
				})
				.sum();
			write_color(&mut pixels, pixel_color, samples_per_pixel);
		}
	}

	eprint!("\rWriting to file...");
	pixels_to_file(&pixels, image_height, image_width, &file_name);
	eprintln!("\nDone.");
	io::stderr().flush().unwrap();
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_render_image_creates_image() {
		let image_width = 100;
		let aspect_ratio = 16.0 / 9.0;
		let file_name = "test.ppm";

		let settings = Settings {
			image_width,
			image_height: calc_height(image_width, aspect_ratio),
			aspect_ratio,
			samples_per_pixel: 1,
			max_depth: 50,
			file_name: file_name.to_string(),
			look_from: Point3::from(13.0, 2.0, 3.0),
			look_at: Point3::from(0.0, 0.0, 0.0),
			focus_distance: 10.0,
			aperture: 0.1,
			field_of_view: 20.0,
		};
		let scene = random_scene();

		render_image(scene, settings);

		std::fs::remove_file(format!("./assets/{}", file_name)).expect("File could not be deleted");
	}
}
