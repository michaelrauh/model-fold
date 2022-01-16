use nohash_hasher::IntMap;
use nohash_hasher::IntSet;
use string_interner::StringInterner;
use string_interner::Symbol;
use std::collections::HashSet;
use std::collections::HashMap;


// todo wherever usize is used in a hashmap or hashset, convert that to an intset or intmap
// wherever hashing is used directly, use a fast hasher
// wherever a hashmap is used, use a hashmap with a fast hasher
// don't use btreemap or btreeset 
pub fn clean_sentences(sentences: String, interner: &mut StringInterner) -> Vec<Vec<usize>> {
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
                .map(|x| StringInterner::get_or_intern(interner, x))
                .map(|x| x.to_usize())
                .collect()
        })
        .collect()
}

pub struct Config {
    vocabulary: IntSet<usize>,
    forward: IntMap<usize, IntSet<usize>>,
    backward: IntMap<usize, IntSet<usize>>,
    // todo add phrases. This will be a map with the hashed together strings in the phrase vector as the keys, and the values as the interned strings they point to
}

struct LiteralConfig {
    vocabulary: HashSet<String>,
    forward: HashMap<String, HashSet<String>>,
    backward: HashMap<String, HashSet<String>>,
    // todo add phrases. This will be a hashmap of vector to set of string
}

impl LiteralConfig {
    fn save() {
        // todo
    }

    fn merge() {
        // todo
    }

    fn intern() {
        // todo
    }

    fn load() {
        // todo
    }
}

impl Config {
    pub fn new(sentences: Vec<Vec<usize>>) -> Config {
        let mut vocabulary = IntSet::default();
        let mut forward = IntMap::default();
        let mut backward = IntMap::default();

        for sentence in sentences {
            for i in 0..sentence.len() - 1 {
                let word = sentence[i];
                let next_word = sentence[i + 1];
                vocabulary.insert(word);
                vocabulary.insert(next_word);

                forward
                    .entry(word)
                    .or_insert_with(IntSet::default)
                    .insert(next_word);

                backward
                    .entry(next_word)
                    .or_insert_with(IntSet::default)
                    .insert(word);
            }
        }
        Config {
            vocabulary,
            forward,
            backward,
        }
    }

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
        let sentences = clean_sentences(raw, &mut interner);
        (Config::new(sentences), interner)
    }
}

#[cfg(test)]
mod tests {
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
}
