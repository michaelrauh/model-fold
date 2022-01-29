use model_fold::step;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    step(filename.to_string());
}
