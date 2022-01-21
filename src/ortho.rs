use std::collections::BTreeMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Ortho {
    nodes: Vec<BTreeMap<MultiSet<usize>, usize>>,
}

impl Ortho {
    pub fn new(a: usize, b: usize, c: usize, d: usize) -> Ortho {
        let mut nodes = vec![
            BTreeMap::default(),
            BTreeMap::default(),
            BTreeMap::default(),
        ];
        let mut b_location = MultiSet::new();
        let mut c_location = MultiSet::new();
        let mut d_location = MultiSet::new();

        b_location.insert(b);
        c_location.insert(c);
        d_location.insert(b);
        d_location.insert(c);

        nodes[0].insert(MultiSet::new(), a);
        nodes[1].insert(b_location, b);
        nodes[1].insert(c_location, c);
        nodes[2].insert(d_location, d);
        Ortho { nodes }
    }

    pub fn unintern() {
        // todo
    }

    pub fn size(&self) -> MultiSet<usize> {
        let mut mapping = self.nodes.last().unwrap().iter();
        let (location, _name) = mapping.next().unwrap(); // todo switch to nightly and use first instead of iter next
        location.size()
    }

    pub fn origin(&self) -> usize {
        let mut mapping = self.nodes.first().unwrap().iter();
        let (_location, name) = mapping.next().unwrap(); // todo switch to nightly and use first instead of iter next
        *name
    }

    pub fn hop(&self) -> std::collections::btree_map::Values<MultiSet<usize>, usize> {
        let mut nodes = self.nodes.iter();
        nodes.next();
        nodes.next().unwrap().values()
    }
}

pub struct LiteralOrtho {
    nodes: Vec<BTreeMap<MultiSet<String>, String>>,
}

impl LiteralOrtho {}

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Clone)]
pub struct MultiSet<T> {
    set: BTreeMap<T, usize>,
}

impl<T: Ord> MultiSet<T> {
    pub fn new() -> MultiSet<T> {
        MultiSet {
            set: BTreeMap::default(),
        }
    }

    pub fn size(&self) -> MultiSet<usize> {
        let mut set = MultiSet::new();
        for (_key, value) in self.set.iter() {
            set.insert(*value);
        }
        set
    }

    pub fn insert(&mut self, item: T) {
        let count = self.set.entry(item).or_insert(0);
        *count += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    #[test]
    fn it_compares_equal_across_rotation() {
        let ortho = Ortho::new(1, 2, 3, 4);
        let ortho2 = Ortho::new(1, 3, 2, 4);

        assert!(ortho == ortho2);
    }

    #[test]
    fn it_exposes_origin() {
        assert_eq!(Ortho::new(1, 2, 3, 4).origin(), 1);
    }

    fn it_exposes_hop() {
        let ortho = Ortho::new(1, 2, 3, 4);
        let actual: Vec<&usize> = ortho.hop().collect();
        let expected = vec![&(2 as usize), &(3 as usize)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_has_size() {
        let mut expected = MultiSet::new();
        expected.insert(1);
        expected.insert(1);
        assert_eq!(Ortho::new(1, 2, 3, 4).size(), expected);
    }
}
