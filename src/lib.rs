use std::io::{self, Write};

use crate::{color::write_color, math::Color};

mod color;
mod math;

pub fn run() {
	let image_height = 256;
	let image_width = 256;

	print!("P3\n{} {}\n255\n", image_width, image_height);
	for j in (0..image_height).rev() {
		eprint!("\rScanlines remaining: {} ", j);
		io::stderr().flush().unwrap();
		for i in 0..image_width {
			let pixel_color = Color::from(
				i as f64 / (image_width - 1) as f64,
				j as f64 / (image_height - 1) as f64,
				0.25,
			);

			write_color(pixel_color);
		}
	}
	eprintln!("\nDone.");
	io::stderr().flush().unwrap();
}
