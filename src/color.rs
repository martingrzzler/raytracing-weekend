use crate::{
	math::{norm, Vec3},
	Ray,
};

pub type Color = Vec3;

impl Color {
	pub fn black() -> Self {
		Color::new()
	}

	pub fn interpolate_by_direction(r: &Ray) -> Self {
		let unit_dir = norm(r.direction());
		let t = 0.5 * (unit_dir.y() + 1.0);

		Color::from(1.0, 1.0, 1.0) * (1.0 - t) + Color::from(0.5, 0.7, 1.0) * t
	}
}
