use crate::math::Vec3;

pub type Color = Vec3;

pub fn write_color(color: Color) {
	println!(
		"{} {} {}",
		(255.999 * color.x()) as i32,
		(255.999 * color.y()) as i32,
		(255.999 * color.z()) as i32
	);
}
