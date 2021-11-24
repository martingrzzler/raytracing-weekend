use crate::{
	math::{clamp, Vec3},
	output::Pixel,
};

pub type Color = Vec3;

pub fn transform_to_pixel(color: Color, samples_per_pixel: i32) -> Pixel {
	let Color {
		e: (mut r, mut g, mut b),
	} = color;

	let scale = 1.0 / samples_per_pixel as f64;

	// sqrt for gamma correction
	r = (scale * r).sqrt();
	g = (scale * g).sqrt();
	b = (scale * b).sqrt();

	Pixel::from(
		(256.0 * clamp(r, 0.0, 0.999)) as i32,
		(256.0 * clamp(g, 0.0, 0.999)) as i32,
		(256.0 * clamp(b, 0.0, 0.999)) as i32,
	)
}
