#![allow(non_upper_case_globals)]
pub use color::Color;
pub use math::Point3;
pub use rendering::random_scene;
pub use rendering::Hit;
pub use rendering::{Lambertian, Sphere};
pub use utils::calc_height;

use std::io::{self, Write};

use color::transform_to_pixel;
use rendering::ray_color;

use crate::camera::CameraProps;
use crate::output::{pixels_to_file, Pixel};
use crate::utils::Progress;
use crate::{camera::Camera, math::rand};

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
			file_name,
			..
		} = &self.settings;

		let progress = &Progress::from(*image_height * *image_width);

		let pixels: Vec<Pixel> = (0..*image_height)
			.into_iter()
			.flat_map(|j| {
				(0..*image_width).into_iter().map(move |i| {
					progress.print();
					transform_to_pixel(self.pixel_color(i, j))
				})
			})
			.collect();

		eprint!("\rWriting to file...");
		pixels_to_file(&pixels, *image_height, *image_width, &file_name);
		eprintln!("\nDone.");
		io::stderr().flush().unwrap();
	}

	fn pixel_color(&self, curr_width: i32, curr_height: i32) -> Color {
		match self.settings.antialiasing {
			Antialiasing::MSAA => {
				let color: Color = (0..self.settings.samples_per_pixel)
					.map(|_sample| {
						let u = (curr_width as f64 + rand()) / ((self.settings.image_width as f64) - 1.0);
						let v = (curr_height as f64 + rand()) / ((self.settings.image_height as f64) - 1.0);
						let r = self.camera.calc_ray(u, v);
						ray_color(&r, &self.scene, self.settings.max_depth)
					})
					.sum();

				let scale = 1.0 / self.settings.samples_per_pixel as f64;
				color * scale
			}
			Antialiasing::NONE => {
				let u = (curr_width as f64) / ((self.settings.image_width as f64) - 1.0);
				let v = (curr_height as f64) / ((self.settings.image_height as f64) - 1.0);
				let r = self.camera.calc_ray(u, v);
				ray_color(&r, &self.scene, self.settings.max_depth)
			}
		}
	}
}

pub enum Antialiasing {
	MSAA,
	NONE,
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
	pub antialiasing: Antialiasing,
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
			antialiasing: Antialiasing::MSAA,
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
			antialiasing: Antialiasing::MSAA,
		};
		let scene = random_scene();

		let renderer = Renderer::from(scene, settings);
		renderer.render();

		// std::fs::remove_file(format!("./assets/{}", file_name)).expect("File could not be deleted");
	}
}
