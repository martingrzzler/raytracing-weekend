pub use entity::hit::*;
pub use entity::sphere::*;
pub use material::{Dielectric, Lambertian, Material, Metal};
pub use ray::Ray;

use crate::color::Color;
use crate::math::{norm, INFINITY};

mod entity;
mod material;
pub mod ray;

pub fn ray_color(r: &Ray, entities: &Vec<Box<dyn Hit>>, depth: i32) -> Color {
	if depth <= 0 {
		return Color::new();
	}
	let opt = trace(&r, 0.001, INFINITY, entities);
	if let Some(rec) = opt {
		if let Some((attenuation, scattered)) = rec.material().scatter(r, rec) {
			return attenuation * ray_color(&scattered, entities, depth - 1);
		}
		return Color::new();
	}
	let unit_dir = norm(r.direction());
	let t = 0.5 * (unit_dir.y() + 1.0);
	let res = Color::from(1.0, 1.0, 1.0) * (1.0 - t) + Color::from(0.5, 0.7, 1.0) * t;
	res
}

fn trace(r: &Ray, t_min: f64, t_max: f64, entities: &Vec<Box<dyn Hit>>) -> Option<HitRecord> {
	let mut closest = t_max;
	let mut rec: Option<HitRecord> = None;
	for e in entities.iter() {
		let opt = e.hit(&r, t_min, closest);
		if let Some(tmp) = opt {
			closest = tmp.t();
			rec = Some(tmp);
		}
	}

	rec
}
