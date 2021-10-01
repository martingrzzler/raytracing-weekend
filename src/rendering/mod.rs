pub use entity::hit::*;
pub use entity::sphere::*;
pub use ray::Ray;

use crate::color::Color;
use crate::math::Vec3;
use crate::math::{norm, INFINITY};

mod entity;
pub mod ray;

pub fn ray_color(r: &Ray, entities: &Vec<Box<dyn Hit>>, depth: i32) -> Color {
	if depth <= 0 {
		Color::new();
	}
	let opt = trace(&r, 0.001, INFINITY, entities);
	if let Some(rec) = opt {
		let target = rec.point() + Vec3::random_in_hemisphere(rec.normal());
		let child_ray = Ray::from(rec.point(), &(target - rec.point()));
		return 0.5 * ray_color(&child_ray, entities, depth - 1);
	}

	let unit_dir = norm(r.direction());
	let t = 0.5 * (unit_dir.y() + 1.0);
	Color::from(1.0, 1.0, 1.0) * (1.0 - t) + Color::from(0.5, 0.7, 1.0) * t
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
