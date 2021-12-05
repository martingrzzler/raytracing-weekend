#![allow(non_upper_case_globals)]
use camera::PlainGenerator;
use camera::RayGenerator;
pub use color::Color;
pub use math::{Point3, Ray};
pub use scene::Hit;
pub use scene::Scene;
pub use scene::{Dielectric, Lambertian, Material, Metal, Sphere};
pub use settings::{
	Antialiasing, CameraSettings, DefocusBlur, ImageSettings, RenderSettings, Settings,
};

pub use utils::{aspect_ratio, calc_height};
pub use writer::{PPMWriter, WriteResult};

use crate::camera::DefocusBlurGenerator;
use crate::pixel::Pixel;
use crate::utils::ProgressBar;
use crate::{camera::Camera, math::rand};
use camera::CameraParams;

use crate::math::{norm, INFINITY};

mod camera;
mod color;
mod math;
mod pixel;
mod scene;
mod settings;
mod utils;
mod writer;

pub struct Renderer {
	scene: Scene,
	settings: Settings,
	ray_generator: Box<dyn RayGenerator>,
}

pub struct RenderingResult {
	pub width: i32,
	pub height: i32,
	pub pixels: Vec<Pixel>,
}

impl Renderer {
	pub fn new() -> Self {
		let settings: Settings = Default::default();
		let ray_generator = Renderer::get_ray_generator(&settings);
		Self {
			scene: Scene::random(),
			settings,
			ray_generator,
		}
	}

	pub fn from(scene: Scene, settings: Settings) -> Self {
		Self {
			scene,
			ray_generator: Renderer::get_ray_generator(&settings),
			settings,
		}
	}

	pub fn render(&self) -> RenderingResult {
		let progress = &ProgressBar::from(self.settings.height() * self.settings.width());
		let pixels: Vec<Pixel> = (0..self.settings.height())
			.into_iter()
			.flat_map(|j| {
				(0..self.settings.width()).into_iter().map(move |i| {
					progress.inc();
					Pixel::from_color(self.pixel_color(i, j))
				})
			})
			.collect();

		RenderingResult {
			width: self.settings.width(),
			height: self.settings.height(),
			pixels,
		}
	}

	fn pixel_color(&self, curr_width: i32, curr_height: i32) -> Color {
		match self.settings.antialiasing() {
			Antialiasing::MSAA { samples_per_pixel } => {
				let color: Color = (0..*samples_per_pixel)
					.map(|_sample| {
						let (s, t) = self.calc_viewport_coordinates(curr_width, curr_height, &rand);
						let r = self.ray_generator.gen_ray(s, t);
						self.trace(&r, self.settings.max_depth())
					})
					.sum();

				color / *samples_per_pixel as f64
			}
			Antialiasing::NONE => {
				let (s, t) = self.calc_viewport_coordinates(curr_width, curr_height, &|| 0.5);
				let r = self.ray_generator.gen_ray(s, t);
				self.trace(&r, self.settings.max_depth())
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

	fn trace(&self, r: &Ray, depth: i32) -> Color {
		if depth <= 0 {
			return Color::black();
		}
		if let Some(rec) = self.scene.intersect(&r, 0.001, INFINITY) {
			if let Some((attenuation, scattered)) = rec.material().scatter(r, rec) {
				return attenuation * self.trace(&scattered, depth - 1);
			}
			return Color::black();
		}

		Color::interpolate_by_direction(r)
	}
}

#[cfg(test)]
mod test {

	use super::*;

	#[test]
	fn test_render_image_creates_image() {
		let path = "./assets/test.ppm";

		let scene = Scene::random();
		let settings = Settings {
			image: ImageSettings {
				width: 150,
				height: 100,
			},
			rendering: RenderSettings {
				antialiasing: Antialiasing::NONE,
				..Default::default()
			},
			..Default::default()
		};
		let writer = PPMWriter::new(path);
		let renderer = Renderer::from(scene, settings);
		let result = renderer.render();
		writer.write(result).expect("Failed writing file");

		std::fs::remove_file(path).expect("File could not be deleted");
	}

	#[test]
	fn test_viewport_coordinates() {
		let width = 100;
		let height = 50;

		let scene = Scene::random();
		let settings = Settings {
			rendering: RenderSettings {
				antialiasing: Antialiasing::NONE,
				..Default::default()
			},
			image: ImageSettings { width, height },
			..Default::default()
		};

		let renderer = Renderer::from(scene, settings);

		let s_t = renderer.calc_viewport_coordinates(0, 0, &|| 0.0);
		assert_eq!(s_t, (0.0, 0.0));

		let s_t = renderer.calc_viewport_coordinates(width - 1, height - 1, &|| 0.0);
		assert_eq!(s_t, (1.0, 1.0));
	}
}
