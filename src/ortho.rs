use std::collections::BTreeMap;
use std::hash::Hash;

use serde::{Deserialize, Serialize};
use string_interner::StringInterner;
use string_interner::Symbol;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Ortho {
    nodes: Vec<BTreeMap<MultiSet, usize>>,
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

    pub fn unintern(&self, interner: &StringInterner) -> LiteralOrtho {
        LiteralOrtho {
            nodes: self
                .nodes
                .iter()
                .map(|m| {
                    m.iter()
                        .map(|(k, v)| {
                            (
                                k.unintern(interner),
                                interner
                                    .resolve(Symbol::try_from_usize(*v).unwrap())
                                    .unwrap()
                                    .to_string(),
                            )
                        })
                        .collect()
                })
                .collect(),
        }
    }

    pub fn size(&self) -> MultiSet {
        let mut mapping = self.nodes.last().unwrap().iter();
        let (location, _name) = mapping.next().unwrap();
        location.size()
    }

    pub fn origin(&self) -> usize {
        let mut mapping = self.nodes.first().unwrap().iter();
        let (_location, name) = mapping.next().unwrap();
        *name
    }

    pub fn hop(&self) -> std::collections::btree_map::Values<MultiSet, usize> {
        let mut nodes = self.nodes.iter();
        nodes.next();
        nodes.next().unwrap().values()
    }
}
#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct LiteralOrtho {
    nodes: Vec<BTreeMap<LiteralMultiSet, String>>,
}

impl LiteralOrtho {
    pub fn intern(&self, interner: &StringInterner) -> Ortho {
        Ortho {
            nodes: self
                .nodes
                .iter()
                .map(|m| {
                    m.iter()
                        .map(|(k, v)| (k.intern(interner), interner.get(v).unwrap().to_usize()))
                        .collect()
                })
                .collect(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct LiteralMultiSet {
    set: BTreeMap<String, usize>,
}

impl LiteralMultiSet {
    pub fn intern(&self, interner: &StringInterner) -> MultiSet {
        MultiSet {
            set: self
                .set
                .iter()
                .map(|(k, v)| (interner.get(k.clone()).unwrap().to_usize(), *v))
                .collect(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct MultiSet {
    set: BTreeMap<usize, usize>,
}

impl MultiSet {
    pub fn unintern(&self, interner: &StringInterner) -> LiteralMultiSet {
        let res = self
            .set
            .iter()
            .map(|(k, v)| {
                (
                    interner
                        .resolve(Symbol::try_from_usize(*k).unwrap())
                        .unwrap()
                        .to_string(),
                    *v,
                )
            })
            .collect();
        LiteralMultiSet { set: res }
    }

    pub fn new() -> MultiSet {
        MultiSet {
            set: BTreeMap::default(),
        }
    }

    pub fn size(&self) -> MultiSet {
        let mut set = MultiSet::new();
        for (_key, value) in self.set.iter() {
            set.insert(*value);
        }
        set
    }

    pub fn insert(&mut self, item: usize) {
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

        assert_eq!(ortho, ortho2);
        assert_eq!(ortho.origin(), ortho2.origin());
        assert_eq!(ortho.size(), ortho2.size());
    }

    #[test]
    fn it_exposes_origin() {
        assert_eq!(Ortho::new(1, 2, 3, 4).origin(), 1);
    }

    #[test]
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
        assert_eq!(Ortho::new(10, 20, 30, 40).size(), expected);
    }

    #[test]
    fn it_has_multisets_that_can_be_interned_or_uninterned() {
        let mut expected = MultiSet::new();
        let mut interner = StringInterner::default();
        expected.insert(interner.get_or_intern("a".to_string()).to_usize());
        expected.insert(interner.get_or_intern("a".to_string()).to_usize());

        assert_eq!(
            expected,
            expected.unintern(&mut interner).intern(&mut interner)
        )
    }

    #[test]
    fn it_can_be_interned_or_uninterned() {
        let mut interner = StringInterner::default();
        let ortho = Ortho::new(
            interner.get_or_intern("a").to_usize(),
            interner.get_or_intern("b").to_usize(),
            interner.get_or_intern("c").to_usize(),
            interner.get_or_intern("d").to_usize(),
        );

        assert_eq!(ortho, ortho.unintern(&mut interner).intern(&mut interner))
    }

    #[test]
    fn multiset_serializes() {
        let mut expected = MultiSet::new();
        let mut interner = StringInterner::default();
        expected.insert(interner.get_or_intern("a".to_string()).to_usize());
        expected.insert(interner.get_or_intern("a".to_string()).to_usize());

        let uninterned = expected.unintern(&mut interner);
        let serialized = serde_yaml::to_string(&uninterned).unwrap();
        let deserialized: LiteralMultiSet = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(uninterned, deserialized);
    }

    #[test]
    fn ortho_serializes() {
        let mut interner = StringInterner::default();

        let ortho = Ortho::new(
            interner.get_or_intern("a").to_usize(),
            interner.get_or_intern("b").to_usize(),
            interner.get_or_intern("c").to_usize(),
            interner.get_or_intern("d").to_usize(),
        );

        let literal_ortho = ortho.unintern(&mut interner);
        let serialized = serde_yaml::to_string(&literal_ortho).unwrap();
        let deserialized: LiteralOrtho = serde_yaml::from_str(&serialized).unwrap();

        assert_eq!(literal_ortho, deserialized);
    }
}
