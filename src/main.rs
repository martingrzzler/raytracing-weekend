use raytracing_weekend::{random_scene, render_image, Settings};

fn main() {
    let scene = random_scene();
    let settings = Settings::new();
    render_image(scene, settings);
}
