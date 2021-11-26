pub use progress::Progress;

pub mod progress;

pub fn calc_height(width: i32, aspect_ratio: f64) -> i32 {
	(width as f64 / aspect_ratio) as i32
}

pub fn aspect_ratio(width: i32, height: i32) -> f64 {
	width as f64 / height as f64
}
