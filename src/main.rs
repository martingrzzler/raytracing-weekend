use raytracing_weekend::{random_scene, DefocusBlur, Renderer, Settings};

fn main() {
    let mut settings: Settings = Default::default();
    settings.rendering.blur = DefocusBlur::OFF;

    let renderer = Renderer::from(random_scene(), settings);
    renderer.render();
}
