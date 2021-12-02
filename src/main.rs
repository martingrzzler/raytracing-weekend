use raytracing_weekend::{
    random_scene, DefocusBlur, ImageSettings, RenderSettings, Renderer, Settings,
};

fn main() {
    let settings = Settings {
        rendering: RenderSettings {
            blur: DefocusBlur::OFF,
            ..Default::default()
        },
        image: ImageSettings {
            width: 200,
            height: 110,
        },
        ..Default::default()
    };

    let renderer = Renderer::from(random_scene(), settings);
    renderer.render();
}
