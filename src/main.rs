use std::env;

use raytracing_weekend::{run, Options};

fn main() {
    let args = env::args().collect();
    let opts = Options {
        ..Default::default()
    };
    run(args, opts);
}
