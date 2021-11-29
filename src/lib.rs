#![allow(non_upper_case_globals)]
use camera::PlainGenerator;
use camera::RayGenerator;
pub use color::Color;
pub use math::Point3;
pub use rendering::random_scene;
pub use rendering::Hit;
pub use rendering::{Lambertian, Sphere};
pub use settings::{Antialiasing, CameraSettings, DefocusBlur, ImageSettings, Settings};
pub use utils::{aspect_ratio, calc_height};

use std::io::{self, Write};

use color::transform_to_pixel;
use rendering::ray_color;

use crate::camera::DefocusBlurGenerator;
use crate::output::{pixels_to_file, Pixel};
use crate::utils::Progress;
use crate::{camera::Camera, math::rand};
use camera::CameraParams;

mod camera;
mod color;
mod math;
mod output;
mod rendering;
mod settings;
mod utils;

pub struct Renderer {
	scene: Vec<Box<dyn Hit>>,
	settings: Settings,
}

impl Renderer {
	pub fn new() -> Self {
		let settings = Default::default();
		Self {
			scene: random_scene(),
			settings,
		}
	}

	pub fn from(scene: Vec<Box<dyn Hit>>, settings: Settings) -> Self {
		Self { settings, scene }
	}

	pub fn render(&self) {
		let ray_generator = &*Renderer::get_ray_generator(&self.settings);
		let progress = &Progress::from(self.settings.height() * self.settings.width());
		let pixels: Vec<Pixel> = (0..self.settings.height())
			.into_iter()
			.flat_map(|j| {
				(0..self.settings.width()).into_iter().map(move |i| {
					progress.print();
					transform_to_pixel(self.pixel_color(ray_generator, i, j))
				})
			})
			.collect();

		eprint!("\rWriting to file...");
		pixels_to_file(
			&pixels,
			self.settings.height(),
			self.settings.width(),
			&self.settings.file_name,
		);
		eprintln!("\nDone.");
		io::stderr().flush().unwrap();
	}

	fn pixel_color(
		&self,
		ray_generator: &dyn RayGenerator,
		curr_width: i32,
		curr_height: i32,
	) -> Color {
		match self.settings.antialiasing() {
			Antialiasing::MSAA { samples_per_pixel } => {
				let color: Color = (0..*samples_per_pixel)
					.map(|_sample| {
						let s = (curr_width as f64 + rand()) / ((self.settings.width() as f64) - 1.0);
						let t = (curr_height as f64 + rand()) / ((self.settings.height() as f64) - 1.0);
						let r = ray_generator.gen_ray(s, t);
						ray_color(&r, &self.scene, self.settings.max_depth())
					})
					.sum();

				let scale = 1.0 / *samples_per_pixel as f64;
				color * scale
			}
			Antialiasing::NONE => {
				let s = (curr_width as f64) / ((self.settings.width() as f64) - 1.0);
				let t = (curr_height as f64) / ((self.settings.height() as f64) - 1.0);
				let r = ray_generator.gen_ray(s, t);
				ray_color(&r, &self.scene, self.settings.max_depth())
			}
		}
	}

	fn get_ray_generator(settings: &Settings) -> Box<dyn RayGenerator> {
		let CameraSettings {
			look_at,
			look_from,
			field_of_view,
		} = settings.camera;
		let cam = Camera::from(CameraParams {
			look_at,
			look_from,
			field_of_view,
			aspect_ratio: aspect_ratio(settings.width(), settings.height()),
		});

		match settings.defocus_blur() {
			&DefocusBlur::OFF => Box::new(PlainGenerator::from(cam)),
			&DefocusBlur::ON {
				aperture,
				focus_distance,
			} => Box::new(DefocusBlurGenerator::from(cam, aperture, focus_distance)),
		}
	}
}

// #[cfg(test)]
// mod test {
// 	use super::*;

// 	#[test]
// 	fn test_render_image_creates_image() {
// 		let image_width = 100;
// 		let aspect_ratio = 16.0 / 9.0;
// 		let file_name = "test.ppm";

// 		let scene = random_scene();

// 		let renderer = Renderer::from(scene, settings);
// 		renderer.render();

// 		// std::fs::remove_file(format!("./assets/{}", file_name)).expect("File could not be deleted");
// 	}
// }
