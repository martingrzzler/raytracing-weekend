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

#[test]
fn test_calc() {
	let progress = Progress {
		curr_height: 5,
		curr_width: 0,
		total_height: 10,
		total_width: 10,
	}
	.calc();

	assert_eq!(progress, 51.0);
}

#[test]
fn test_curr_pixel() {
	let progress = Progress {
		curr_height: 1,
		curr_width: 2,
		total_height: 100,
		total_width: 200,
	}
	.curr_pixel();

	assert_eq!(progress, 203);
}
