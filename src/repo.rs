use crate::ortho::LiteralOrtho;
use crate::ortho::MultiSet;
use crate::ortho::Ortho;
use std::collections::BTreeSet;
use std::collections::HashMap;

pub struct Repo {
    origin: HashMap<(MultiSet<usize>, usize), BTreeSet<Ortho>>,
    hops: HashMap<(MultiSet<usize>, usize), BTreeSet<Ortho>>,
}

pub struct LiteralRepo {
    origin: HashMap<(MultiSet<String>, String), LiteralOrtho>,
    hops: HashMap<(MultiSet<String>, String), LiteralOrtho>,
}

impl Repo {
    pub fn len(&self) -> usize {
        self.origin.len()
    }

    pub fn find_by_size_and_origin(
        &self,
        size: MultiSet<usize>,
        origin: usize,
    ) -> Option<&BTreeSet<Ortho>> {
        self.origin.get(&(size, origin))
    }

    pub fn find_by_size_and_hop(
        &self,
        size: MultiSet<usize>,
        origin: usize,
    ) -> Option<&BTreeSet<Ortho>> {
        self.hops.get(&(size, origin))
    }

    pub fn new() -> Repo {
        Repo {
            origin: HashMap::default(),
            hops: HashMap::default(),
        }
    }

    pub fn unintern() {
        // todo converts to uninterned repo
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

impl LiteralRepo {
    pub fn intern() {
        // todo
    }

    pub fn save() {
        // todo
    }

    pub fn load() {
        // todo
    }
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
}
