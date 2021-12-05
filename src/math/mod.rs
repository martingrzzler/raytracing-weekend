#![allow(dead_code)]
pub use ray::Ray;
pub use std::f64::consts::PI;
pub use std::f64::INFINITY;
pub use vec3::{cross, dot, norm, reflect, refract, Point3, Vec3};

use rand_distr::{Distribution, Uniform};

pub mod ray;
pub mod vec3;

pub fn radians(degrees: f64) -> f64 {
	degrees * PI / 180.0
}

pub fn rand() -> f64 {
	let mut rng = rand::thread_rng();
	let uniform_dst = Uniform::new_inclusive(0.0, 1.0);

	uniform_dst.sample(&mut rng)
}

pub fn rand_rng(min: f64, max: f64) -> f64 {
	let mut rng = rand::thread_rng();
	let uniform_dst = Uniform::new_inclusive(min, max);

	uniform_dst.sample(&mut rng)
}

#[inline]
pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
	debug_assert!(min <= max, "min must be less than or equal to max");
	if input < min {
		min
	} else if input > max {
		max
	} else {
		input
	}
}

#[cfg(test)]
mod test {

	use super::*;

	#[test]
	fn test_degress_to_radians() {
		let degrees = 90.0;
		assert_eq!(PI / 2.0, radians(degrees));
	}
}
