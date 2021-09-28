#![allow(dead_code)]
pub use std::f64::consts::PI;
pub use std::f64::INFINITY;
pub use vec3::{cross, dot, norm, Color, Point3, Vec3};

pub mod vec3;

pub fn radians(degrees: f64) -> f64 {
	degrees * PI / 180.0
}
