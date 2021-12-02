use std::rc::Rc;

pub use entity::hit::*;
pub use entity::sphere::*;
pub use material::{Dielectric, Lambertian, Material, Metal};
pub use ray::Ray;

use crate::math::rand;
use crate::math::{norm, Point3, INFINITY};
use crate::Color;

mod entity;
mod material;
pub mod ray;

pub fn trace(r: &Ray, scene: &Vec<Box<dyn Hit>>, depth: i32) -> Color {
	if depth <= 0 {
		return Color::new();
	}
	let opt = intersect(&r, 0.001, INFINITY, scene);
	if let Some(rec) = opt {
		if let Some((attenuation, scattered)) = rec.material().scatter(r, rec) {
			return attenuation * trace(&scattered, scene, depth - 1);
		}
		return Color::new();
	}
	let unit_dir = norm(r.direction());
	let t = 0.5 * (unit_dir.y() + 1.0);

	Color::from(1.0, 1.0, 1.0) * (1.0 - t) + Color::from(0.5, 0.7, 1.0) * t
}

fn intersect(r: &Ray, t_min: f64, t_max: f64, scene: &Vec<Box<dyn Hit>>) -> Option<HitRecord> {
	let mut closest = t_max;
	let mut rec: Option<HitRecord> = None;
	for e in scene.iter() {
		let opt = e.hit(&r, t_min, closest);
		if let Some(tmp) = opt {
			closest = tmp.t();
			rec = Some(tmp);
		}
	}

	rec
}

pub fn random_scene() -> Vec<Box<dyn Hit>> {
	let mut scene: Vec<Box<dyn Hit>> = vec![];
	let ground_mat = Lambertian::from(Color::from(0.5, 0.5, 0.5));
	let ground = Box::new(Sphere::from(
		Point3::from(0.0, -1000.0, 0.0),
		1000.0,
		Rc::new(ground_mat),
	));
	scene.push(ground);

	for a in -11..11 {
		for b in -11..11 {
			let choose_mat = rand();
			let center = Point3::from(a as f64 + 0.9 * rand(), 0.2, b as f64 + 0.9 * rand());

			if (center - Point3::from(4.0, 0.2, 0.0)).len() > 0.9 {
				if choose_mat < 0.8 {
					// diffuse
					let albedo = Color::from_rand() * Color::from_rand();
					let mat = Lambertian::from(albedo);
					scene.push(Box::new(Sphere::from(center, 0.2, Rc::new(mat))));
				} else if choose_mat < 0.95 {
					// metal
					let albedo = Color::from_rand_rng(0.5, 1.0);
					let fuzz = rand();
					let mat = Metal::from(albedo, fuzz);
					scene.push(Box::new(Sphere::from(center, 0.2, Rc::new(mat))))
				} else {
					// glass
					let mat = Dielectric::from(1.5);
					scene.push(Box::new(Sphere::from(center, 0.2, Rc::new(mat))))
				}
			}
		}
	}

	let mat = Dielectric::from(1.5);
	scene.push(Box::new(Sphere::from(
		Point3::from(0.0, 1.0, 0.0),
		1.0,
		Rc::new(mat),
	)));
	let mat = Lambertian::from(Color::from(0.4, 0.2, 0.1));
	scene.push(Box::new(Sphere::from(
		Point3::from(-4.0, 1.0, 0.0),
		1.0,
		Rc::new(mat),
	)));
	let mat = Metal::from(Color::from(0.7, 0.6, 0.5), 0.0);
	scene.push(Box::new(Sphere::from(
		Point3::from(4.0, 1.0, 0.0),
		1.0,
		Rc::new(mat),
	)));

	scene
}
