pub use progress::Progress;

pub mod progress;

pub fn calc_height(width: i32, aspect_ratio: f64) -> i32 {
	(width as f64 / aspect_ratio) as i32
}
