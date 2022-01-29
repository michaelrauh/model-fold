use crate::ortho::LiteralOrtho;
use crate::ortho::MultiSet;
use crate::ortho::Ortho;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use string_interner::StringInterner;
use string_interner::Symbol;

#[derive(Debug, PartialEq, Eq)]
pub struct Repo {
    origin: HashMap<(MultiSet, usize), BTreeSet<Ortho>>,
    hops: HashMap<(MultiSet, usize), BTreeSet<Ortho>>,
}

impl Repo {
    pub fn set_subract(&self, from: BTreeSet<Ortho>) -> BTreeSet<Ortho> {
        from.iter()
            .filter(|&o| self.find_by_size_and_origin(o.size(), o.origin()).is_none())
            .cloned()
            .collect()
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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
    ) -> HashMap<(MultiSet, String), BTreeSet<LiteralOrtho>> {
        hm.iter()
            .map(|(k, v)| {
                (
                    (
                        k.0.clone(),
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct LiteralRepo {
    origin: HashMap<(MultiSet, String), BTreeSet<LiteralOrtho>>,
    hops: HashMap<(MultiSet, String), BTreeSet<LiteralOrtho>>,
}

impl LiteralRepo {
    pub fn intern(&self, interner: &StringInterner) -> Repo {
        Repo {
            origin: Self::intern_underlying(&self.origin, interner),
            hops: Self::intern_underlying(&self.hops, interner),
        }
    }

    fn intern_underlying(
        underlying: &HashMap<(MultiSet, String), BTreeSet<LiteralOrtho>>,
        interner: &StringInterner,
    ) -> HashMap<(MultiSet, usize), BTreeSet<Ortho>> {
        underlying
            .iter()
            .map(|(k, v)| {
                (
                    (k.0.clone(), interner.get(k.1.clone()).unwrap().to_usize()),
                    v.iter().map(|v| v.intern(interner)).collect(),
                )
            })
            .collect()
    }

    pub fn save(&self, mut target: File) {
        std::io::Write::write_fmt(
            &mut target,
            format_args!("{}", serde_yaml::to_string(&self).unwrap()),
        )
        .unwrap();
    }

    pub fn load(mut source: File) -> LiteralRepo {
        let mut contents = String::new();
        source.read_to_string(&mut contents).unwrap();
        serde_yaml::from_str(&contents).unwrap()
    }

    pub fn merge(&mut self, other: LiteralRepo) {
        for x in other.origin.iter() {
            self.origin
                .entry(x.0.clone())
                .or_insert(BTreeSet::default())
                .extend(x.1.iter().cloned());
        }

        for x in other.hops.iter() {
            self.hops
                .entry(x.0.clone())
                .or_insert(BTreeSet::default())
                .extend(x.1.iter().cloned());
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

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

    #[test]
    fn it_can_be_used_to_subtract() {
        let mut interner = StringInterner::default();
        let mut repo = Repo::new();
        let ortho = Ortho::new(
            interner.get_or_intern("a").to_usize(),
            interner.get_or_intern("b").to_usize(),
            interner.get_or_intern("c").to_usize(),
            interner.get_or_intern("d").to_usize(),
        );

        let ortho2 = Ortho::new(
            interner.get_or_intern("e").to_usize(),
            interner.get_or_intern("f").to_usize(),
            interner.get_or_intern("g").to_usize(),
            interner.get_or_intern("h").to_usize(),
        );

        repo.add(ortho.clone());

        let mut target = BTreeSet::default();
        target.insert(ortho.clone());
        target.insert(ortho2.clone());

        let res = repo.set_subract(target);
        assert_eq!(res.len(), 1);
        assert!(res.contains(&ortho2));
    }

    #[test]
    fn it_can_be_merged() {
        let mut interner = StringInterner::default();
        let mut repo = Repo::new();
        let mut repo2 = Repo::new();
        let ortho = Ortho::new(
            interner.get_or_intern("a").to_usize(),
            interner.get_or_intern("b").to_usize(),
            interner.get_or_intern("c").to_usize(),
            interner.get_or_intern("d").to_usize(),
        );

        let ortho2 = Ortho::new(
            interner.get_or_intern("e").to_usize(),
            interner.get_or_intern("f").to_usize(),
            interner.get_or_intern("g").to_usize(),
            interner.get_or_intern("h").to_usize(),
        );

        repo.add(ortho.clone());
        repo2.add(ortho2.clone());

        let mut literal_repo = repo.unintern(&interner);
        let literal_repo2 = repo2.unintern(&interner);

        literal_repo.merge(literal_repo2.clone());
        let res = literal_repo.intern(&mut interner);

        assert_eq!(res.origin.len(), 2);
        assert_eq!(repo.origin.len(), 1);
        assert_eq!(repo2.origin.len(), 1);
        assert_eq!(res.hops.len(), 4);
        assert_eq!(repo.hops.len(), 2);
        assert_eq!(repo2.hops.len(), 2);
    }

    #[test]
    fn it_saves_and_loads() {
        let filename = "temp_two.yaml";
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

        let f = File::create(filename).unwrap();
        uninterned.save(f);

        let f2 = File::open(filename).unwrap();

        let res = LiteralRepo::load(f2);
        fs::remove_file(filename).unwrap();
        assert_eq!(uninterned, res);
    }
}
