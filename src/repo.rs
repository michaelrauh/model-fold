use crate::ortho::LiteralOrtho;
use crate::ortho::Ortho;
use nohash_hasher::IntMap;
use nohash_hasher::IntSet;
use string_interner::StringInterner;
use string_interner::Symbol;
use std::collections::HashSet;
use std::collections::HashMap;

pub struct Repo {
    Origin: HashMap<usize, Ortho>,
    Hops: HashMap<usize, Ortho>,
}

pub struct LiteralRepo {
    Origin: HashMap<String, LiteralOrtho>,
    Hops: HashMap<String, LiteralOrtho>,
}

impl Repo {
    pub fn new() { 
        // todo makes an interned repo
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