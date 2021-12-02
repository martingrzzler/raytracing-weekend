pub use entity::hit::*;
pub use entity::sphere::*;
pub use material::{Dielectric, Lambertian, Material, Metal};
pub use ray::Ray;

mod entity;
mod material;
pub mod ray;
