use string_interner::StringInterner;
use string_interner::Symbol;
use nohash_hasher::IntMap;
use nohash_hasher::IntSet;

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

// enum Trie {
//     Cons(HashMap<usize, Box<Trie>>),
//     Nil,
// }

// struct Trie {
//     Cons: HashMap<usize, Trie>,
// }

enum Trie {
    Cons(IntMap<usize, Trie>)
}

fn foo() {
    let mut bar = IntMap::default();
    let to_insert: Trie = Trie::Cons(IntMap::default());
    bar.insert(1, to_insert);
}
pub struct Config {
    vocabulary: IntSet<usize>,
    forward: IntMap<usize, IntSet<usize>>,
    backward: IntMap<usize, IntSet<usize>>,
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

    pub fn new(sentences: Vec<Vec<usize>>) -> Config {
        let mut vocabulary = IntSet::default();
        let mut forward = IntMap::default();
        let mut backward = IntMap::default();

        for sentence in sentences {
            for word in sentence.clone() {
                vocabulary.insert(word);
            }

            for i in 0..sentence.len() - 1 {
                let word = *&sentence[i].to_usize() as usize;
                let next_word = *&sentence[i + 1].to_usize() as usize;

                forward
                    .entry(word.clone())
                    .or_insert_with(IntSet::default)
                    .insert(next_word.clone());

                backward
                    .entry(next_word.clone())
                    .or_insert_with(IntSet::default)
                    .insert(word.clone());
            }
        }

        Config {
            vocabulary,
            forward,
            backward,
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
        assert!(
            config.project_forward(interner.get("a").unwrap().to_usize()).unwrap().contains(&interner.get("b").unwrap().to_usize())
        );

        assert!(
            config.project_forward(interner.get("a").unwrap().to_usize()).unwrap().contains(&interner.get("c").unwrap().to_usize())
        );

        assert_eq!(
            config.project_forward(interner.get("a").unwrap().to_usize()).unwrap().len(), 2
        );
    }

    #[test]
    fn it_projects_backward() {
        let raw = "a b. c d. a c. b d.".to_string();
        let mut interner = StringInterner::default();
        let sentences = clean_sentences(raw, &mut interner);
        let config = Config::new(sentences);
        assert!(
            config.project_backward(interner.get("b").unwrap().to_usize()).unwrap().contains(&interner.get("a").unwrap().to_usize())
        );

        assert_eq!(
            config.project_backward(interner.get("b").unwrap().to_usize()).unwrap().len(), 1
        );
    }
}
