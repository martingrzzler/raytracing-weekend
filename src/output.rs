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
