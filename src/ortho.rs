use std::hash::Hash;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ortho {
    nodes: Vec<BTreeMap<MultiSet<usize>, usize>>
}

pub struct LiteralOrtho {
    nodes: Vec<BTreeMap<MultiSet<String>, String>>,
}

pub struct LiteralNode {}

impl Ortho {
    pub fn new(a: usize, b: usize, c: usize, d: usize) -> Ortho {
        let mut nodes = vec![BTreeMap::default(), BTreeMap::default(), BTreeMap::default()];
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
}

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct MultiSet<T> {
    set: BTreeMap<T, usize>,
}

impl<T: Ord> MultiSet<T> {
    pub fn new() -> MultiSet<T> {
        MultiSet { set: BTreeMap::default() }
    }

    pub fn insert(&mut self, item: T) {
        let count = self.set.entry(item).or_insert(0);
        *count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_compares_equal_across_rotation() {
        let ortho = Ortho::new(1, 2, 3, 4);
        let ortho2 = Ortho::new(1, 3, 2, 4);

        assert!(ortho == ortho2);
    }
}
