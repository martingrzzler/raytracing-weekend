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

pub struct Renderer {
	scene: Vec<Box<dyn Hit>>,
	camera: Camera,
	settings: Settings,
}

impl Renderer {
	pub fn new() -> Self {
		let settings = Settings::new();
		Self {
			scene: random_scene(),
			camera: Camera::from(CameraProps {
				field_of_view: settings.field_of_view,
				look_at: settings.look_at,
				look_from: settings.look_from,
				focus_distance: settings.focus_distance,
				aperture: settings.aperture,
				aspect_ratio: settings.aspect_ratio,
			}),
			settings,
		}
	}

	pub fn from(scene: Vec<Box<dyn Hit>>, settings: Settings) -> Self {
		let camera = Camera::from(CameraProps {
			field_of_view: settings.field_of_view,
			look_at: settings.look_at,
			look_from: settings.look_from,
			focus_distance: settings.focus_distance,
			aperture: settings.aperture,
			aspect_ratio: settings.aspect_ratio,
		});

		Self {
			settings,
			scene,
			camera,
		}
	}

	pub fn render(&self) {
		let Settings {
			image_height,
			image_width,
			samples_per_pixel,
			file_name,
			..
		} = &self.settings;

		let mut pixels = vec![];
		for j in 0..*image_height {
			for i in 0..*image_width {
				Progress {
					curr_height: j,
					curr_width: i,
					total_height: *image_height,
					total_width: *image_width,
				}
				.print();
				write_color(&mut pixels, self.pixel_color(i, j), *samples_per_pixel);
			}
		}

		eprint!("\rWriting to file...");
		pixels_to_file(&pixels, *image_height, *image_width, &file_name);
		eprintln!("\nDone.");
		io::stderr().flush().unwrap();
	}

	fn pixel_color(&self, curr_width: i32, curr_height: i32) -> Color {
		(0..self.settings.samples_per_pixel)
			.map(|_sample| {
				let u = (curr_width as f64 + rand()) / ((self.settings.image_width as f64) - 1.0);
				let v = (curr_height as f64 + rand()) / ((self.settings.image_height as f64) - 1.0);
				let r = self.camera.calc_ray(u, v);
				ray_color(&r, &self.scene, self.settings.max_depth)
			})
			.sum()
	}
}

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

		let renderer = Renderer::from(scene, settings);
		renderer.render();

		std::fs::remove_file(format!("./assets/{}", file_name)).expect("File could not be deleted");
	}
}
