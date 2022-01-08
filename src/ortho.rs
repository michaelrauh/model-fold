use std::collections::HashSet;
use std::hash::Hash;

pub struct Ortho<'a> {
    nodes: Vec<HashSet<Node<'a>>>,
}

impl Ortho<'_> {
    pub fn new<'a>(a: &'a str, b: &'a str, c: &'a str, d: &'a str) -> Ortho<'a> {
        let mut nodes = vec![HashSet::new(), HashSet::new(), HashSet::new()];
        let a_set = MultiSet::new();
        let mut b_set = MultiSet::new();
        let mut c_set = MultiSet::new();
        let mut d_set = MultiSet::new();

        b_set.insert(b);
        c_set.insert(c);
        d_set.insert(b);
        d_set.insert(d);

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

#[derive(PartialEq, Eq, Hash)]
pub struct Node<'a> {
    name: &'a str,
    location: MultiSet<'a>,
}

#[derive(PartialEq, Eq, Hash)]
pub struct MultiSet<'a> {
    set: Vec<&'a str>,
}

impl<'a> MultiSet<'a> {
    pub fn new() -> MultiSet<'a> {
        MultiSet { set: vec![] }
    }

    pub fn insert(&mut self, item: &'a str) {
        self.set.push(item);
        self.set.sort();
    }
}
