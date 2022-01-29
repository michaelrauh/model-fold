mod config;
mod ex_nihilo;
use std::fs;

pub use config::Config;

mod ortho;
mod repo;
mod search;

pub use ortho::Ortho;

pub fn step(input: String) {
    search::search(
        fs::read_to_string(input).unwrap(),
        "config.yaml",
        "repo.yaml",
    );
}
