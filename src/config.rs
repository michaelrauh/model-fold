use nohash_hasher::IntMap;
use nohash_hasher::IntSet;
use std::collections::HashMap;
use std::collections::HashSet;
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

pub struct LiteralConfig {
    vocabulary: HashSet<String>,
    forward: HashMap<String, HashSet<String>>,
    backward: HashMap<String, HashSet<String>>,
}

impl LiteralConfig {
    pub fn from_raw(raw: String) -> LiteralConfig {
        Self::new(clean_sentences(raw))
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

    fn save() {}

    fn merge() {}

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

    fn load() {}
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
