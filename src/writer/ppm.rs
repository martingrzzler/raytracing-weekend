use core::fmt;
use std::fmt::Write;
use std::io::Write as IoWrite;
use std::{fmt::Display, fs::File};

use super::WriteResult;
use crate::RenderingResult;

pub struct PPMWriter {
	path: String,
	magic_number: MagicNumber,
	max_color: i32,
}

enum MagicNumber {
	P3,
}

impl PPMWriter {
	pub fn new(path: &str) -> Self {
		Self {
			path: path.to_string(),
			magic_number: MagicNumber::P3,
			max_color: 255,
		}
	}
}

impl WriteResult for PPMWriter {
	fn write(
		&self,
		RenderingResult {
			width,
			height,
			pixels,
		}: RenderingResult,
	) -> Result<(), Box<dyn std::error::Error>> {
		let mut file = File::create(&self.path)?;
		let mut out = format!(
			"{}\n{} {}\n{}\n",
			self.magic_number.to_string(),
			width,
			height,
			self.max_color,
		);
		for p in pixels.iter() {
			writeln!(out, "{} {} {}", p.r(), p.g(), p.b())?;
		}

		file.write_all(out.as_bytes())?;

		Ok(())
	}
}

impl Display for MagicNumber {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&MagicNumber::P3 => write!(f, "P3"),
		}
	}
}
