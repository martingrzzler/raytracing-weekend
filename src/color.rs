use crate::{
	math::{clamp, Vec3},
	output::Pixel,
};

pub type Color = Vec3;

pub fn transform_to_pixel(color: Color) -> Pixel {
	let Color { e: (r, g, b) } = gamma_correct(color);

	Pixel::from(
		(256.0 * clamp(r, 0.0, 0.999)) as i32,
		(256.0 * clamp(g, 0.0, 0.999)) as i32,
		(256.0 * clamp(b, 0.0, 0.999)) as i32,
	)
}

pub fn gamma_correct(color: Color) -> Color {
	Color {
		e: (color.x().sqrt(), color.y().sqrt(), color.z().sqrt()),
	}
}
