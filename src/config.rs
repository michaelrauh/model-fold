use nohash_hasher::IntMap;
use nohash_hasher::IntSet;
use string_interner::StringInterner;
use string_interner::Symbol;

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

#[derive(Debug, PartialEq)]
pub struct Trie {
    get: IntMap<usize, Trie>,
}

impl Trie {
    pub fn phrase_hop(&self, word: usize) -> Option<&Trie> {
        self.get.get(&word)
    }

    fn new() -> Trie {
        Trie {
            get: IntMap::default(),
        }
    }
}
pub struct Config {
    vocabulary: IntSet<usize>,
    forward: IntMap<usize, IntSet<usize>>,
    backward: IntMap<usize, IntSet<usize>>,
    phrases: Trie,
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

    pub fn get_phrases(&self) -> &Trie {
        &self.phrases
    }

    // todo: when saving these in a DB or merging them, beware that interning is inconistent (rebuild interning or rebuild config)
    pub fn new(sentences: Vec<Vec<usize>>) -> Config {
        println!("Building config...");
        let mut vocabulary = IntSet::default();
        let mut forward = IntMap::default();
        let mut backward = IntMap::default();
        let mut phrases = Trie::new();

        for sentence in sentences {
            for word in sentence.clone() {
                vocabulary.insert(word);
            }

            for i in 0..sentence.len() - 1 {
                let word = sentence[i];
                let next_word = sentence[i + 1];

                forward
                    .entry(word)
                    .or_insert_with(IntSet::default)
                    .insert(next_word);

                backward
                    .entry(next_word)
                    .or_insert_with(IntSet::default)
                    .insert(word);
            }

            let mut current_trie = &mut phrases;
            for i in (0..(sentence.len() - 1)).rev() {
                let word = sentence[i];
                let next_word = sentence[i + 1];
                println!("{} {}", word, next_word);

                current_trie
                    .get
                    .entry(word)
                    .or_insert_with(|| Trie::new())
                    .get
                    .entry(next_word)
                    .or_insert_with(|| Trie::new());

                current_trie = current_trie.get.get_mut(&word).unwrap();
                println!("{:?}", current_trie);
            }
        }
        println!("{:?}", phrases);
        Config {
            vocabulary,
            forward,
            backward,
            phrases,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_iterates() {
        let raw = "a b. c d. a c. b d.".to_string();
        let mut interner = StringInterner::default();
        let sentences = clean_sentences(raw, &mut interner);
        let config = Config::new(sentences);
        assert_eq!(config.vocabulary.len(), 4);
        assert_eq!(config.iter().collect::<Vec<_>>().len(), 4);
        for word in config.iter() {
            assert!(config.vocabulary.contains(word));
        }
    }

    #[test]
    fn it_projects_forward() {
        let raw = "a b. c d. a c. b d.".to_string();
        let mut interner = StringInterner::default();
        let sentences = clean_sentences(raw, &mut interner);
        let config = Config::new(sentences);
        assert!(config
            .project_forward(interner.get("a").unwrap().to_usize())
            .unwrap()
            .contains(&interner.get("b").unwrap().to_usize()));

        assert!(config
            .project_forward(interner.get("a").unwrap().to_usize())
            .unwrap()
            .contains(&interner.get("c").unwrap().to_usize()));

        assert_eq!(
            config
                .project_forward(interner.get("a").unwrap().to_usize())
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    fn it_projects_backward() {
        let raw = "a b. c d. a c. b d.".to_string();
        let mut interner = StringInterner::default();
        let sentences = clean_sentences(raw, &mut interner);
        let config = Config::new(sentences);
        assert!(config
            .project_backward(interner.get("b").unwrap().to_usize())
            .unwrap()
            .contains(&interner.get("a").unwrap().to_usize()));

        assert_eq!(
            config
                .project_backward(interner.get("b").unwrap().to_usize())
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn it_hops_phrases_backward() {
        let raw = "a b. c d. a c. b d.".to_string();
        let mut interner = StringInterner::default();
        let sentences = clean_sentences(raw, &mut interner);
        let config = Config::new(sentences);
        assert!(config
            .get_phrases()
            .phrase_hop(interner.get("b").unwrap().to_usize())
            .unwrap()
            .phrase_hop(interner.get("a").unwrap().to_usize())
            .is_none());
    }
}
