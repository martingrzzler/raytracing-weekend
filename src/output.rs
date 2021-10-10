use std::env;
use std::fmt::Write;
use std::fs::File;
use std::io::Write as IoWrite;
pub struct Pixel {
	r: i32,
	g: i32,
	b: i32,
}

impl Pixel {
	pub fn from(r: i32, g: i32, b: i32) -> Self {
		Self { r, g, b }
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

pub fn pixels_to_file(pixels: Vec<Pixel>, image_height: i32, image_width: i32) {
	let args: Vec<String> = env::args().collect();
	let path = format!("./assets/{}", args[1]);
	let mut file = File::create(path).expect("File creation failed");
	let mut out = format!("P3\n{} {}\n255\n", image_width, image_height);
	for p in pixels.iter() {
		writeln!(out, "{} {} {}", p.r(), p.g(), p.b()).expect("Failed to write Pixel");
	}

	file
		.write_all(out.as_bytes())
		.expect("Failed while writing to file");
}
