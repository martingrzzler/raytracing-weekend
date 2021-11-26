use std::rc::Rc;

use raytracing_weekend::{
    calc_height, random_scene, Antialiasing, Color, Hit, Lambertian, Point3, Renderer, Settings,
    Sphere,
};

fn main() {
    let image_width = 500;
    let aspect_ratio = 16.0 / 9.0;
    let file_name = "mmm.ppm";

    let mut scene: Vec<Box<dyn Hit>> = vec![];
    let material = Lambertian::from(Color::from(1.0, 0.0, 0.0));
    scene.push(Box::new(Sphere::from(
        Point3::from(0.0, 0.0, -3.0),
        2.0,
        Rc::new(material),
    )));

    let settings = Settings {
        image_width,
        image_height: calc_height(image_width, aspect_ratio),
        aspect_ratio,
        samples_per_pixel: 50,
        max_depth: 50,
        file_name: file_name.to_string(),
        look_from: Point3::from(0.0, 0.0, 0.0),
        look_at: Point3::from(0.0, 0.0, -1.0),
        focus_distance: 1.0,
        aperture: 0.001,
        field_of_view: 90.0,
        antialiasing: Antialiasing::MSAA,
    };

    let renderer = Renderer::from(scene, settings);
    renderer.render();
}
