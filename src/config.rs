use nohash_hasher::IntMap;
use nohash_hasher::IntSet;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use string_interner::StringInterner;
use string_interner::Symbol;

pub fn clean_sentences(sentences: String) -> Vec<Vec<String>> {
    sentences
        .split(|x| x == '.' || x == '!' || x == '?')
        .filter(|x| !x.is_empty())
        .map(|sentence| {
            sentence
                .replace(";", "")
                .replace("\'", "")
                .replace(":", "")
                .replace(",", "")
                .to_lowercase()
                .split_ascii_whitespace()
                .map(|x| x.to_string())
                .collect()
        })
        .collect()
}

pub struct Config {
    vocabulary: IntSet<usize>,
    forward: IntMap<usize, IntSet<usize>>,
    backward: IntMap<usize, IntSet<usize>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct LiteralConfig {
    vocabulary: HashSet<String>,
    forward: HashMap<String, HashSet<String>>,
    backward: HashMap<String, HashSet<String>>,
}

impl LiteralConfig {
    pub fn from_raw(raw: String) -> LiteralConfig {
        let res = Self::new(clean_sentences(raw));
        res
    }

    pub fn new(sentences: Vec<Vec<String>>) -> LiteralConfig {
        let mut vocabulary = HashSet::default();
        let mut forward = HashMap::default();
        let mut backward = HashMap::default();

        for sentence in sentences {
            for i in 0..sentence.len() - 1 {
                let word = sentence[i].clone();
                let next_word = sentence[i + 1].clone();
                vocabulary.insert(word.clone());
                vocabulary.insert(next_word.clone());

                forward
                    .entry(word.clone())
                    .or_insert_with(HashSet::default)
                    .insert(next_word.clone());

                backward
                    .entry(next_word)
                    .or_insert_with(HashSet::default)
                    .insert(word);
            }
        }
        LiteralConfig {
            vocabulary,
            forward,
            backward,
        }
    }

    pub fn save(&self, mut target: File) {
        std::io::Write::write_fmt(
            &mut target,
            format_args!("{}", serde_yaml::to_string(&self).unwrap()),
        )
        .unwrap();
    }

    pub fn merge(&mut self, other: LiteralConfig) {
        for x in other.vocabulary {
            self.vocabulary.insert(x);
        }

        for (k, v) in other.forward {
            self.forward
                .entry(k)
                .or_insert_with(HashSet::default)
                .extend(v);
        }

        for (k, v) in other.backward {
            self.backward
                .entry(k)
                .or_insert_with(HashSet::default)
                .extend(v);
        }
    }

    pub fn intern(&self, string_interner: &mut StringInterner) -> Config {
        Config {
            vocabulary: Self::intern_hashset(&self.vocabulary, string_interner),
            forward: Self::intern_hashmap(&self.forward, string_interner),
            backward: Self::intern_hashmap(&self.backward, string_interner),
        }
    }

    fn intern_hashset(hs: &HashSet<String>, string_interner: &mut StringInterner) -> IntSet<usize> {
        let mut set = IntSet::default();
        for word in hs.iter() {
            set.insert(string_interner.get_or_intern(word).to_usize());
        }
        set
    }

    fn intern_hashmap(
        hm: &HashMap<String, HashSet<String>>,
        string_interner: &mut StringInterner,
    ) -> IntMap<usize, IntSet<usize>> {
        let mut new_hm = IntMap::default();
        for (k, v) in hm {
            new_hm.insert(
                string_interner.get_or_intern(k).to_usize(),
                Self::intern_hashset(v, string_interner),
            );
        }
        new_hm
    }

    pub fn load(mut source: File) -> LiteralConfig {
        let mut contents = String::new();
        source.read_to_string(&mut contents).unwrap();
        serde_yaml::from_str(&contents).unwrap()
    }
}

impl Config {
    pub fn project_forward(&self, word: usize) -> Option<&IntSet<usize>> {
        self.forward.get(&word)
    }

    pub fn project_backward(&self, word: usize) -> Option<&IntSet<usize>> {
        self.backward.get(&word)
    }

    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.vocabulary.iter()
    }

    pub fn from_sentences(raw: String) -> (Config, StringInterner) {
        let mut interner = StringInterner::default();
        let config = LiteralConfig::new(clean_sentences(raw)).intern(&mut interner);
        (config, interner)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs
    };

    use super::*;

    fn string_to_usize(interner: &StringInterner, string: &str) -> usize {
        interner.get(string).unwrap().to_usize()
    }

    #[test]
    fn it_iterates() {
        let (config, _interner) = Config::from_sentences("a b. c d. a c. b d.".to_string());
        assert_eq!(config.vocabulary.len(), 4);
        assert_eq!(config.iter().collect::<Vec<_>>().len(), 4);
        for word in config.iter() {
            assert!(config.vocabulary.contains(word));
        }
    }

    #[test]
    fn it_projects_forward() {
        let (config, interner) = Config::from_sentences("a b. c d. a c. b d.".to_string());
        assert!(config
            .project_forward(string_to_usize(&interner, "a"))
            .unwrap()
            .contains(&string_to_usize(&interner, "b")));

        assert!(config
            .project_forward(string_to_usize(&interner, "a"))
            .unwrap()
            .contains(&string_to_usize(&interner, "c")));

        assert_eq!(
            config
                .project_forward(string_to_usize(&interner, "a"))
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    fn it_projects_backward() {
        let (config, interner) = Config::from_sentences("a b. c d. a c. b d.".to_string());
        assert!(config
            .project_backward(string_to_usize(&interner, "b"))
            .unwrap()
            .contains(&string_to_usize(&interner, "a")));

        assert_eq!(
            config
                .project_backward(string_to_usize(&interner, "b"))
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn it_merges() {
        let mut literal_config = LiteralConfig::from_raw("a b. c d. a c. b d.".to_string());
        let literal_config2 = LiteralConfig::from_raw("e f. g h. e g. f h.".to_string());
        let mut interner = StringInterner::default();
        let config = literal_config.intern(&mut interner);
        let config2 = literal_config2.intern(&mut interner);

        literal_config.merge(literal_config2);
        let res = literal_config.intern(&mut interner);

        assert_eq!(res.vocabulary.len(), 8);
        assert_eq!(config.vocabulary.len(), 4);
        assert_eq!(config2.vocabulary.len(), 4);
        assert_eq!(res.forward.len(), 6);
        assert_eq!(config.forward.len(), 3);
        assert_eq!(config2.forward.len(), 3);
        assert_eq!(res.backward.len(), 6);
        assert_eq!(config.backward.len(), 3);
        assert_eq!(config2.backward.len(), 3);
    }

    #[test]
    fn it_saves_and_loads() {
        let filename = "temp.yaml";
        let literal_config = LiteralConfig::from_raw("a b. c d. a c. b d.".to_string());

        let f = File::create(filename).unwrap();
        literal_config.save(f);

        let f2 = File::open(filename).unwrap();

        let res = LiteralConfig::load(f2);
        fs::remove_file(filename).unwrap();
        assert_eq!(literal_config, res);
    }
}
