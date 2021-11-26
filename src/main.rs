use std::rc::Rc;

use raytracing_weekend::{
    calc_height, random_scene, Antialiasing, Color, Hit, Lambertian, Point3, Renderer, Settings,
    Sphere,
};

fn main() {
    let settings = Default::default();

    let renderer = Renderer::from(random_scene(), settings);
    renderer.render();
}
