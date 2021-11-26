use std::rc::Rc;

use raytracing_weekend::{
    calc_height, random_scene, Antialiasing, Color, Hit, Lambertian, Point3, Renderer, Settings,
    Sphere,
};

fn main() {
    let mut scene: Vec<Box<dyn Hit>> = vec![];
    let material = Lambertian::from(Color::from(1.0, 0.0, 0.0));
    scene.push(Box::new(Sphere::from(
        Point3::from(0.0, 0.0, -3.0),
        2.0,
        Rc::new(material),
    )));

    let settings = Default::default();

    let renderer = Renderer::from(scene, settings);
    renderer.render();
}
