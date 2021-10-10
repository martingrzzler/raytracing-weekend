#![allow(non_upper_case_globals)]
use std::io::{self, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;

use color::{write_color, Color};
use rayon::ThreadPoolBuilder;
use rendering::ray_color;

use crate::output::pixels_to_file;
use crate::rendering::random_scene;
use crate::{
	camera::Camera,
	math::{rand, Point3, Vec3},
	output::Pixel,
};

mod camera;
mod color;
mod math;
mod output;
mod rendering;

pub fn run() {
	// Processing
	let pool = ThreadPoolBuilder::new()
		.num_threads(20)
		.spawn_handler(|thread| {
			std::thread::spawn(|| thread.run());
			Ok(())
		})
		.build()
		.unwrap();
	let (tx, rx): (Sender<(usize, Color)>, Receiver<(usize, Color)>) = channel();

	// Image
	const aspect_ratio: f64 = 16.0 / 9.0;
	const image_width: i32 = 1200;
	const image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
	let samples_per_pixel = 500;
	let max_depth = 50;

	// Entities
	let entities = Arc::new(random_scene());

	// Camera
	let look_from = Point3::from(13.0, 2.0, 3.0);
	let look_at = Point3::from(0.0, 0.0, 0.0);
	let vup = Vec3::from(0.0, 1.0, 0.0);
	let focus_distance = 10.0;
	let aperture = 0.1;
	let cam = Camera::new(
		look_from,
		look_at,
		vup,
		20.0,
		aspect_ratio,
		aperture,
		focus_distance,
	);
	let cam = Arc::new(cam);

	// Render
	for j in 0..image_height {
		for i in 0..image_width {
			eprint!(
				"\rProgress: {:.2}%",
				((j * image_width + i + 1) as f64 / ((image_height * image_width) as f64) * 100.0)
			);
			io::stderr().flush().unwrap();
			let tx = tx.clone();
			let cam = Arc::clone(&cam);
			let entities = Arc::clone(&entities);
			pool.install(move || {
				let mut pixel_color = Color::new();
				for _ in 0..samples_per_pixel {
					let u = (i as f64 + rand()) / ((image_width as f64) - 1.0);
					let v = (j as f64 + rand()) / ((image_height as f64) - 1.0);
					let r = cam.calc_ray(u, v);
					pixel_color += ray_color(&r, &*entities, max_depth);
				}
				tx.send(((j * image_width + i) as usize, pixel_color))
					.expect("Sending Color data from thread failed");
			});
		}
	}
	let mut pixels = vec![Pixel::new(); (image_height * image_width) as usize];
	for _ in 0..(image_height * image_width) {
		let (index, color) = rx.recv().unwrap();
		write_color(&mut *pixels, index, color, samples_per_pixel);
	}

	eprint!("\rWriting to file...");
	pixels_to_file(&*pixels, image_height, image_width);
	eprintln!("\nDone.");
	io::stderr().flush().unwrap();
}
