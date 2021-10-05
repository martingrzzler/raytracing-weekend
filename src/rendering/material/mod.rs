use crate::color::Color;
use crate::rendering::ray::Ray;

use super::HitRecord;
pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

mod dielectric;
mod lambertian;
mod metal;

pub trait Material {
	fn scatter(&self, r_in: &Ray, rec: HitRecord) -> Option<(Color, Ray)>;
}
