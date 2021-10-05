use std::rc::Rc;

use crate::math::{dot, Point3, Vec3};
use crate::rendering::{Material, Ray};

pub struct HitRecord {
	p: Point3,
	normal: Vec3,
	t: f64,
	front_face: bool,
	mat: Rc<dyn Material>,
}

impl HitRecord {
	pub fn new(mat: Rc<dyn Material>) -> Self {
		Self {
			p: Point3::new(),
			t: 0.0,
			normal: Vec3::new(),
			front_face: false,
			mat,
		}
	}

	pub fn normal(&self) -> &Vec3 {
		&self.normal
	}

	pub fn front_face(&self) -> bool {
		self.front_face
	}

	pub fn material(&self) -> Rc<dyn Material> {
		Rc::clone(&self.mat)
	}

	pub fn t_mut(&mut self) -> &mut f64 {
		&mut self.t
	}

	pub fn t(&self) -> f64 {
		self.t
	}

	pub fn point(&self) -> &Point3 {
		&self.p
	}

	pub fn point_mut(&mut self) -> &mut Point3 {
		&mut self.p
	}

	pub fn calc_normal(&mut self, r: &Ray, outward_normal: Vec3) {
		self.front_face = dot(r.direction(), &outward_normal) < 0.0;
		self.normal = if self.front_face {
			outward_normal
		} else {
			-outward_normal
		}
	}
}

pub trait Hit {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
