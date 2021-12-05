use raytracing_weekend::{
    Antialiasing, DefocusBlur, ImageSettings, PPMWriter, RenderSettings, Renderer, Scene, Settings,
    WriteResult,
};

fn main() {
    let settings = Settings {
        image: ImageSettings {
            width: 400,
            height: 280,
        },
        rendering: RenderSettings {
            antialiasing: Antialiasing::NONE,
            ..Default::default()
        },
        ..Default::default()
    };

    let renderer = Renderer::from(Scene::random(), settings);
    let result = renderer.render();

    let writer = PPMWriter::new("./assets/default.ppm");
    writer.write(result).expect("Failed writing file");
}
