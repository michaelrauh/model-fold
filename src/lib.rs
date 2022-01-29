mod config;
mod ex_nihilo;
pub use config::Config;

mod ortho;
mod repo;
mod search;

pub use ortho::Ortho;

pub fn step(input: String) {
    search::search(input, "config.yaml", "repo.yaml");
}
