use model_fold::step;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    step(query.to_string());
}
