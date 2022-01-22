use crate::ex_nihilo::create;
use crate::ortho::LiteralOrtho;
use crate::repo::Repo;
use crate::Config;
use crate::Ortho;
use string_interner::StringInterner;
use string_interner::Symbol;
use crate::repo::LiteralRepo;
use crate::config::LiteralConfig;

pub fn search(input: String) -> (LiteralRepo, StringInterner, LiteralConfig) {
    let mut interner = StringInterner::default();
    let literal_config = LiteralConfig::from_raw(input);
    let config = literal_config.intern(&mut interner);

    let mut repo = Repo::new();
    for a in config.iter() {
        for find in create(&config, *a) {
            repo.add(find);
        }
    }
    
    (repo.unintern(&interner), interner, literal_config)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_finds_atoms() {
        // todo change to a bootstrap method. If there are no saved things, start from nothing. Save results. There can be one method at the end of the day. consume.
        let (literal_repo, mut interner, _config) = search("a b. c d. a c. b d.".to_string());
        let to_find = Ortho::new(
            interner.get("a").unwrap().to_usize(),
            interner.get("b").unwrap().to_usize(),
            interner.get("c").unwrap().to_usize(),
            interner.get("d").unwrap().to_usize(),
        );
        let repo = literal_repo.intern(&mut interner);
        let actual = repo
            .find_by_size_and_origin(to_find.size(), to_find.origin())
            .unwrap()
            .iter()
            .next()
            .unwrap();

        assert_eq!(repo.len(), 1);
        assert_eq!(*actual, to_find);
    }

    // todo it loads a repo and config, merges them, saves the plain config, searches, uninterns the repo, and saves it.
}
