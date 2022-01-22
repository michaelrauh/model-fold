use crate::ex_nihilo::create;
use crate::ortho::LiteralOrtho;
use crate::repo::Repo;
use crate::Config;
use crate::Ortho;
use string_interner::StringInterner;
use string_interner::Symbol;

pub fn search(input: String) -> (Repo, StringInterner, Config) {
    let (config, interner) = Config::from_sentences(input);
    let mut repo = Repo::new();
    for a in config.iter() {
        for find in create(&config, *a) {
            repo.add(find);
        }
    }

    (repo, interner, config)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_finds_atoms() {
        // todo change to a bootstrap method. If there are no saved things, start from nothing. Save results. Give back literals and encapsulate interned things
        let (repo, interner, _config) = search("a b. c d. a c. b d.".to_string());
        let to_find = Ortho::new(
            interner.get("a").unwrap().to_usize(),
            interner.get("b").unwrap().to_usize(),
            interner.get("c").unwrap().to_usize(),
            interner.get("d").unwrap().to_usize(),
        );
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
