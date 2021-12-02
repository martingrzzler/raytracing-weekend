use crate::RenderingResult;
pub use ppm::PPMWriter;

mod ppm;

pub trait WriteResult {
	fn write(&self, result: RenderingResult) -> Result<(), Box<dyn std::error::Error>>;
}
