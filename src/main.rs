use raytracing_weekend::{
    random_scene, DefocusBlur, ImageSettings, PPMWriter, RenderSettings, Renderer, Settings,
    WriteResult,
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
    let result = renderer.render();

    let writer = PPMWriter::new("./assets/default.ppm");
    writer.write(result).expect("Failed writing file");
}
