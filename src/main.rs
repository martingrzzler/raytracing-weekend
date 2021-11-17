use std::env;

use raytracing_weekend::run;

fn main() {
    let args = env::args().collect();
    run(args);
}
