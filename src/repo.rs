use crate::ortho::LiteralMultiSet;
use crate::ortho::LiteralOrtho;
use crate::ortho::MultiSet;
use crate::ortho::Ortho;
use std::collections::BTreeSet;
use std::collections::HashMap;
use string_interner::StringInterner;
use string_interner::Symbol;

#[derive(Debug, PartialEq, Eq)]
pub struct Repo {
    origin: HashMap<(MultiSet, usize), BTreeSet<Ortho>>,
    hops: HashMap<(MultiSet, usize), BTreeSet<Ortho>>,
}

impl Repo {
    pub fn len(&self) -> usize {
        self.origin.len()
    }

    pub fn find_by_size_and_origin(
        &self,
        size: MultiSet,
        origin: usize,
    ) -> Option<&BTreeSet<Ortho>> {
        self.origin.get(&(size, origin))
    }

    pub fn find_by_size_and_hop(&self, size: MultiSet, origin: usize) -> Option<&BTreeSet<Ortho>> {
        self.hops.get(&(size, origin))
    }

    pub fn new() -> Repo {
        Repo {
            origin: HashMap::default(),
            hops: HashMap::default(),
        }
    }

    pub fn unintern(&self, interner: &StringInterner) -> LiteralRepo {
        LiteralRepo {
            origin: Self::unintern_hashmap(&self.origin, interner),
            hops: Self::unintern_hashmap(&self.hops, interner),
        }
    }

    fn unintern_hashmap(
        hm: &HashMap<(MultiSet, usize), BTreeSet<Ortho>>,
        interner: &StringInterner,
    ) -> HashMap<(LiteralMultiSet, String), BTreeSet<LiteralOrtho>> {
        hm.iter()
            .map(|(k, v)| {
                (
                    (
                        k.0.unintern(interner),
                        interner
                            .resolve(Symbol::try_from_usize(k.1).unwrap())
                            .unwrap()
                            .to_string(),
                    ),
                    v.iter().map(|v| v.unintern(interner)).collect(),
                )
            })
            .collect()
    }

    pub fn add(&mut self, ortho: Ortho) {
        self.origin
            .entry((ortho.size(), ortho.origin()))
            .or_insert(BTreeSet::default())
            .insert(ortho.clone());

        for hop in ortho.clone().hop() {
            self.hops
                .entry((ortho.size(), *hop))
                .or_insert(BTreeSet::default())
                .insert(ortho.clone());
        }
    }
}

pub struct LiteralRepo {
    origin: HashMap<(LiteralMultiSet, String), BTreeSet<LiteralOrtho>>,
    hops: HashMap<(LiteralMultiSet, String), BTreeSet<LiteralOrtho>>,
}

impl LiteralRepo {
    pub fn intern(&self, interner: &mut StringInterner) -> Repo {
        Repo {
            origin: Self::intern_underlying(&self.origin, interner),
            hops: Self::intern_underlying(&self.hops, interner),
        }
    }

    fn intern_underlying(
        underlying: &HashMap<(LiteralMultiSet, String), BTreeSet<LiteralOrtho>>,
        interner: &mut StringInterner,
    ) -> HashMap<(MultiSet, usize), BTreeSet<Ortho>> {
        underlying
            .iter()
            .map(|(k, v)| {
                (
                    (
                        k.0.intern(interner),
                        interner.get(k.1.clone()).unwrap().to_usize(),
                    ),
                    v.iter().map(|v| v.intern(interner)).collect(),
                )
            })
            .collect()
    }

    pub fn save(&self) {}

    pub fn load(&self) {}

    pub fn merge(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_size() {
        let mut repo = Repo::new();
        let ortho = Ortho::new(1, 2, 3, 4);
        repo.add(ortho.clone());
        let res = repo.len();
        assert_eq!(res, 1);
    }

    #[test]
    fn it_can_be_found_by_size_and_origin() {
        let mut repo = Repo::new();
        let ortho = Ortho::new(1, 2, 3, 4);
        repo.add(ortho.clone());
        let res = repo
            .find_by_size_and_origin(ortho.size(), ortho.origin())
            .unwrap()
            .iter()
            .next()
            .unwrap();
        assert_eq!(*res, ortho);
    }

    #[test]
    fn it_can_be_found_by_size_and_hop() {
        let mut repo = Repo::new();
        let ortho = Ortho::new(1, 2, 3, 4);
        repo.add(ortho.clone());

        for hop in ortho.hop() {
            let res = repo
                .find_by_size_and_hop(ortho.size(), *hop)
                .unwrap()
                .iter()
                .next()
                .unwrap();
            assert_eq!(*res, ortho);
        }
    }

    #[test]
    fn it_can_be_uninterned_and_reinterned() {
        let mut interner = StringInterner::default();
        let mut repo = Repo::new();
        let ortho = Ortho::new(
            interner.get_or_intern("a").to_usize(),
            interner.get_or_intern("b").to_usize(),
            interner.get_or_intern("c").to_usize(),
            interner.get_or_intern("d").to_usize(),
        );

        repo.add(ortho.clone());

        let uninterned = repo.unintern(&interner);
        let back = uninterned.intern(&mut interner);

        assert_eq!(back, repo);
    }
}
