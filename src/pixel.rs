use crate::math::clamp;
use crate::Color;

#[derive(Copy, Clone)]
pub struct Pixel {
	r: i32,
	g: i32,
	b: i32,
}

impl Pixel {
	pub fn from(r: i32, g: i32, b: i32) -> Self {
		Self { r, g, b }
	}
	pub fn new() -> Self {
		Self { r: 0, b: 0, g: 0 }
	}

	pub fn r(&self) -> i32 {
		self.r
	}
	pub fn g(&self) -> i32 {
		self.g
	}
	pub fn b(&self) -> i32 {
		self.b
	}
}

impl Pixel {
	pub fn from_color(color: Color) -> Pixel {
		let Color { e: (r, g, b) } = Pixel::gamma_correct(color);

		Pixel::from(
			(256.0 * clamp(r, 0.0, 0.999)) as i32,
			(256.0 * clamp(g, 0.0, 0.999)) as i32,
			(256.0 * clamp(b, 0.0, 0.999)) as i32,
		)
	}

	fn gamma_correct(color: Color) -> Color {
		Color {
			e: (color.x().sqrt(), color.y().sqrt(), color.z().sqrt()),
		}
	}
}
