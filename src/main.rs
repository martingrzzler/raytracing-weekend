use std::env;

use raytracing_weekend::{run, Settings};

fn main() {
    let args = env::args().collect();
    let settings = Settings::new();
    run(args, settings);
}
