use std::fs::File;

use crate::config::LiteralConfig;
use crate::ex_nihilo::create;
use crate::repo::LiteralRepo;
use crate::repo::Repo;
use crate::Config;
use string_interner::StringInterner;

pub fn search(input: String, config_filename: &str, repo_filename: &str) {
    if std::path::Path::new(config_filename).exists() {
        let mut interner = StringInterner::default();
        let mut literal_config = LiteralConfig::from_raw(input);
        let config = literal_config.intern(&mut interner);
        let mut repo = Repo::new();

        make_atoms(&config, &mut repo);

        let mut literal_repo = repo.unintern(&interner);

        let (old_config, old_repo) = load_from_disk(config_filename, repo_filename);

        literal_repo.merge(old_repo);
        literal_config.merge(old_config);

        let mut new_interner = StringInterner::default();

        let current_config = literal_config.intern(&mut new_interner);
        let mut current_repo = literal_repo.intern(&mut new_interner);

        make_atoms(&current_config, &mut current_repo);

        save_to_disk(
            &mut literal_config,
            &mut new_interner,
            &mut current_repo,
            config_filename,
            repo_filename,
        );
    } else {
        let mut interner = StringInterner::default();
        let mut literal_config = LiteralConfig::from_raw(input);
        let config = literal_config.intern(&mut interner);

        let mut repo = Repo::new();
        make_atoms(&config, &mut repo);
        save_to_disk(
            &mut literal_config,
            &mut interner,
            &mut repo,
            config_filename,
            repo_filename,
        );
    }
}

fn load_from_disk(config_filename: &str, repo_filename: &str) -> (LiteralConfig, LiteralRepo) {
    let old_config = LiteralConfig::load(File::open(config_filename).unwrap());
    let old_repo = LiteralRepo::load(File::open(repo_filename).unwrap());
    (old_config, old_repo)
}

fn save_to_disk(
    literal_config: &mut LiteralConfig,
    new_interner: &mut StringInterner,
    current_repo: &mut Repo,
    config_filename: &str,
    repo_filename: &str,
) {
    current_repo
        .unintern(&new_interner)
        .save(File::create(repo_filename).unwrap());
    literal_config.save(File::create(config_filename).unwrap());
}

fn make_atoms(config: &Config, repo: &mut Repo) {
    for a in config.iter() {
        for find in create(&config, &repo, *a) {
            repo.add(find);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_advances() {
        let config_filename = "test_config.yaml";
        let repo_filename = "test_repo.yaml";
        search(
            "a b. c d. a c. b d. i k. j l.".to_string(),
            config_filename,
            repo_filename,
        );
        let mut first_interner = StringInterner::default();
        LiteralConfig::load(File::open(config_filename).unwrap()).intern(&mut first_interner);
        let first_repo =
            LiteralRepo::load(File::open(repo_filename).unwrap()).intern(&first_interner);

        assert_eq!(first_repo.len(), 1);

        search(
            "e f. g h. e g. f h. i j. k l.".to_string(),
            config_filename,
            repo_filename,
        );

        let mut second_interner = StringInterner::default();
        LiteralConfig::load(File::open(config_filename).unwrap()).intern(&mut second_interner);
        let second_repo =
            LiteralRepo::load(File::open(repo_filename).unwrap()).intern(&second_interner);

        assert_eq!(second_repo.len(), 3);

        std::fs::remove_file(config_filename).unwrap();
        std::fs::remove_file(repo_filename).unwrap();
    }
}
