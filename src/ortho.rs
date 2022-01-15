use std::collections::{BTreeSet, HashSet};
use std::hash::Hash;

#[derive(Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Ortho {
    nodes: Vec<BTreeSet<Node>>,
}

impl Ortho {
    pub fn new(a: usize, b: usize, c: usize, d: usize) -> Ortho {
        let mut nodes = vec![BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];
        let a_set = MultiSet::new();
        let mut b_set = MultiSet::new();
        let mut c_set = MultiSet::new();
        let mut d_set = MultiSet::new();

        b_set.insert(b);
        c_set.insert(c);
        d_set.insert(b);
        d_set.insert(c);

        let node_a = Node {
            name: a,
            location: a_set,
        };

        let node_b = Node {
            name: b,
            location: b_set,
        };

        let node_c = Node {
            name: c,
            location: c_set,
        };

        let node_d = Node {
            name: d,
            location: d_set,
        };

        nodes[0].insert(node_a);
        nodes[1].insert(node_b);
        nodes[1].insert(node_c);
        nodes[2].insert(node_d);
        Ortho { nodes }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Node {
    name: usize,
    location: MultiSet,
}

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct MultiSet {
    set: Vec<usize>,
}

// todo make this faster. Probably use a map of counts
impl MultiSet {
    pub fn new() -> MultiSet {
        MultiSet { set: vec![] }
    }

    pub fn insert(&mut self, item: usize) {
        self.set.push(item);
        self.set.sort();
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
