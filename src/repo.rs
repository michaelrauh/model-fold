use crate::ortho::LiteralOrtho;
use crate::ortho::Ortho;
use std::collections::HashMap;

pub struct Repo {
    origin: HashMap<usize, Ortho>,
    hops: HashMap<usize, Ortho>,
}

pub struct LiteralRepo {
    origin: HashMap<String, LiteralOrtho>,
    hops: HashMap<String, LiteralOrtho>,
}

impl Repo {
    pub fn len(&self) -> usize {
        0
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

    pub fn add() {
        // todo
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
