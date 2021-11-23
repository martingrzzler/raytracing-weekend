use std::io::{self, Write};

pub struct Progress {
	pub curr_width: i32,
	pub curr_height: i32,
	pub total_height: i32,
	pub total_width: i32,
}

impl Progress {
	fn calc(&self) -> f64 {
		(self.curr_pixel() as f64 / (self.total_height * self.total_width) as f64) * 100.0
	}

	fn curr_pixel(&self) -> i32 {
		self.curr_height * self.total_width + self.curr_width + 1
	}

	pub fn print(&self) {
		eprint!("\rProgress: {:.2}%", self.calc());
		io::stderr().flush().unwrap();
	}
}

pub fn calc_height(width: i32, aspect_ratio: f64) -> i32 {
	(width as f64 / aspect_ratio) as i32
}
