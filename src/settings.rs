use crate::math::Point3;
use std::default::Default;

pub enum Antialiasing {
	MSAA { samples_per_pixel: i32 },
	NONE,
}
pub struct ImageSettings {
	pub width: i32,
	pub height: i32,
}

pub enum DefocusBlur {
	ON { focus_distance: f64, aperture: f64 },
	OFF,
}

pub struct RenderSettings {
	pub max_depth: i32,
	pub antialiasing: Antialiasing,
	pub blur: DefocusBlur,
}

pub struct CameraSettings {
	pub look_at: Point3,
	pub look_from: Point3,
	pub field_of_view: f64,
}

pub struct Settings {
	pub rendering: RenderSettings,
	pub camera: CameraSettings,
	pub image: ImageSettings,
}

impl Settings {
	pub fn width(&self) -> i32 {
		self.image.width
	}

	pub fn height(&self) -> i32 {
		self.image.height
	}

	pub fn antialiasing(&self) -> &Antialiasing {
		&self.rendering.antialiasing
	}

	pub fn max_depth(&self) -> i32 {
		self.rendering.max_depth
	}

	pub fn defocus_blur(&self) -> &DefocusBlur {
		&self.rendering.blur
	}
}

impl Default for RenderSettings {
	fn default() -> Self {
		Self {
			max_depth: 50,
			antialiasing: Antialiasing::MSAA {
				samples_per_pixel: 50,
			},
			blur: DefocusBlur::ON {
				focus_distance: 10.0,
				aperture: 0.1,
			},
		}
	}
}

impl Default for CameraSettings {
	fn default() -> Self {
		Self {
			look_at: Point3::from(0.0, 0.0, 0.0),
			look_from: Point3::from(13.0, 2.0, 3.0),
			field_of_view: 20.0,
		}
	}
}

impl Default for ImageSettings {
	fn default() -> Self {
		Self {
			width: 720,
			height: 576,
		}
	}
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			rendering: Default::default(),
			camera: Default::default(),
			image: Default::default(),
		}
	}
}
