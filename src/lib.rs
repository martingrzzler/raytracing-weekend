#![allow(non_upper_case_globals)]
use camera::PlainGenerator;
use camera::RayGenerator;
pub use color::Color;
pub use math::Point3;
pub use rendering::random_scene;
pub use rendering::Hit;
pub use rendering::{Lambertian, Sphere};
pub use settings::{
	Antialiasing, CameraSettings, DefocusBlur, ImageSettings, RenderSettings, Settings,
};
pub use utils::{aspect_ratio, calc_height};

use std::io::{self, Write};

use color::transform_to_pixel;
use rendering::trace;

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
	ray_generator: Box<dyn RayGenerator>,
}

impl Renderer {
	pub fn new() -> Self {
		let settings: Settings = Default::default();
		let ray_generator = Renderer::get_ray_generator(&settings);
		Self {
			scene: random_scene(),
			settings,
			ray_generator,
		}
	}

	pub fn from(scene: Vec<Box<dyn Hit>>, settings: Settings) -> Self {
		Self {
			scene,
			ray_generator: Renderer::get_ray_generator(&settings),
			settings,
		}
	}

	pub fn render(&self) {
		let progress = &Progress::from(self.settings.height() * self.settings.width());
		let pixels: Vec<Pixel> = (0..self.settings.height())
			.into_iter()
			.flat_map(|j| {
				(0..self.settings.width()).into_iter().map(move |i| {
					progress.print();
					transform_to_pixel(self.pixel_color(i, j))
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

	fn pixel_color(&self, curr_width: i32, curr_height: i32) -> Color {
		match self.settings.antialiasing() {
			Antialiasing::MSAA { samples_per_pixel } => {
				let color: Color = (0..*samples_per_pixel)
					.map(|_sample| {
						let (s, t) = self.calc_viewport_coordinates(curr_width, curr_height, &rand);
						let r = self.ray_generator.gen_ray(s, t);
						trace(&r, &self.scene, self.settings.max_depth())
					})
					.sum();

				color / *samples_per_pixel as f64
			}
			Antialiasing::NONE => {
				let (s, t) = self.calc_viewport_coordinates(curr_width, curr_height, &|| 0.5);
				let r = self.ray_generator.gen_ray(s, t);
				trace(&r, &self.scene, self.settings.max_depth())
			}
		}
	}

	fn calc_viewport_coordinates(
		&self,
		curr_width: i32,
		curr_height: i32,
		deviation: &dyn Fn() -> f64,
	) -> (f64, f64) {
		let s: f64 = (curr_width as f64 + deviation()) / ((self.settings.width() as f64) - 1.0);
		let t: f64 = (curr_height as f64 + deviation()) / ((self.settings.height() as f64) - 1.0);
		(s, t)
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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_render_image_creates_image() {
		let file_name = "test.ppm";

		let scene = random_scene();
		let settings = Settings {
			image: ImageSettings {
				width: 150,
				height: 100,
			},
			rendering: RenderSettings {
				antialiasing: Antialiasing::NONE,
				..Default::default()
			},
			file_name: file_name.to_string(),
			..Default::default()
		};

		let renderer = Renderer::from(scene, settings);
		renderer.render();

		std::fs::remove_file(format!("./assets/{}", file_name)).expect("File could not be deleted");
	}

	#[test]
	fn test_viewport_coordinates() {
		let file_name = "test.ppm";
		let width = 100;
		let height = 50;

		let scene = random_scene();
		let settings = Settings {
			rendering: RenderSettings {
				antialiasing: Antialiasing::NONE,
				..Default::default()
			},
			image: ImageSettings { width, height },
			file_name: file_name.to_string(),
			..Default::default()
		};

		let renderer = Renderer::from(scene, settings);

		let s_t = renderer.calc_viewport_coordinates(0, 0, &|| 0.0);
		assert_eq!(s_t, (0.0, 0.0));

		let s_t = renderer.calc_viewport_coordinates(width - 1, height - 1, &|| 0.0);
		assert_eq!(s_t, (1.0, 1.0));
	}
}
