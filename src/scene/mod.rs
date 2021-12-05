use crate::math::Ray;
pub use entity::hit::*;
pub use entity::sphere::*;
pub use entity::Entity;
pub use material::{Dielectric, Lambertian, Material, Metal};
use std::rc::Rc;

mod entity;
mod material;

use crate::math::rand;
use crate::math::Point3;
use crate::Color;

pub struct Scene {
	entities: Vec<Entity>,
}

impl Scene {
	pub fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let mut closest = t_max;
		let mut rec: Option<HitRecord> = None;
		for e in self.entities.iter() {
			let opt = e.hit(&r, t_min, closest);
			if let Some(tmp) = opt {
				closest = tmp.t();
				rec = Some(tmp);
			}
		}

		rec
	}

	pub fn add_entity(&mut self, entity: Entity) {
		self.entities.push(entity);
	}

	pub fn random() -> Scene {
		let mut entities: Vec<Entity> = vec![];
		let ground_mat = Lambertian::from(Color::from(0.5, 0.5, 0.5));
		let ground = Box::new(Sphere::from(
			Point3::from(0.0, -1000.0, 0.0),
			1000.0,
			Rc::new(ground_mat),
		));
		entities.push(ground);

		for a in -11..11 {
			for b in -11..11 {
				let choose_mat = rand();
				let center = Point3::from(a as f64 + 0.9 * rand(), 0.2, b as f64 + 0.9 * rand());

				if (center - Point3::from(4.0, 0.2, 0.0)).len() > 0.9 {
					if choose_mat < 0.8 {
						// diffuse
						let albedo = Color::from_rand() * Color::from_rand();
						let mat = Lambertian::from(albedo);
						entities.push(Box::new(Sphere::from(center, 0.2, Rc::new(mat))));
					} else if choose_mat < 0.95 {
						// metal
						let albedo = Color::from_rand_rng(0.5, 1.0);
						let fuzz = rand();
						let mat = Metal::from(albedo, fuzz);
						entities.push(Box::new(Sphere::from(center, 0.2, Rc::new(mat))))
					} else {
						// glass
						let mat = Dielectric::from(1.5);
						entities.push(Box::new(Sphere::from(center, 0.2, Rc::new(mat))))
					}
				}
			}
		}

		let mat = Dielectric::from(1.5);
		entities.push(Box::new(Sphere::from(
			Point3::from(0.0, 1.0, 0.0),
			1.0,
			Rc::new(mat),
		)));
		let mat = Lambertian::from(Color::from(0.4, 0.2, 0.1));
		entities.push(Box::new(Sphere::from(
			Point3::from(-4.0, 1.0, 0.0),
			1.0,
			Rc::new(mat),
		)));
		let mat = Metal::from(Color::from(0.7, 0.6, 0.5), 0.0);
		entities.push(Box::new(Sphere::from(
			Point3::from(4.0, 1.0, 0.0),
			1.0,
			Rc::new(mat),
		)));

		Self { entities }
	}
}
