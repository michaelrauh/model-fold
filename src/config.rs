use std::collections::HashMap;
use std::collections::HashSet;

fn clean_sentences(sentences: String) -> Vec<Vec<String>> {
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

// enum Trie {
//     Cons(HashMap<u64, Box<Trie>>),
//     Nil,
// }

// struct Trie {
//     Cons: HashMap<u64, Trie>,
// }

enum Trie {
    Cons(HashMap<u64, Trie>)
}

fn foo() {
    let mut bar = HashMap::new();
    let to_insert: Trie = Trie::Cons(HashMap::default());
    bar.insert(1, to_insert);
}

// todo: use string interning on everything. Then use an intmap to hold them, with a no-hash hasher. 

pub struct Config {
    vocabulary: HashSet<String>,
    forward: HashMap<String, HashSet<String>>,
    backward: HashMap<String, HashSet<String>>,
}

impl Config {
    pub fn project_forward(&self, word: &str) -> Option<&HashSet<String>> {
        self.forward.get(word)
    }

    pub fn project_backward(&self, word: &str) -> Option<&HashSet<String>> {
        self.backward.get(word)
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.vocabulary.iter()
    }

    pub fn new(input: String) -> Config {
        let sentences = clean_sentences(input);

        let mut vocabulary = HashSet::new();
        let mut forward = HashMap::new();
        let mut backward = HashMap::new();

        for sentence in sentences {
            for word in sentence.clone() {
                vocabulary.insert(word.clone());
            }

            for i in 0..sentence.len() - 1 {
                let word = &sentence[i];
                let next_word = &sentence[i + 1];

                forward
                    .entry(word.clone())
                    .or_insert_with(HashSet::new)
                    .insert(next_word.clone());

                backward
                    .entry(next_word.clone())
                    .or_insert_with(HashSet::new)
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
    use maplit::hashset;
    #[test]
    fn it_iterates() {
        let config = Config::new("a b. c d. a c. b d.".to_string());
        assert_eq!(config.vocabulary.len(), 4);
        assert_eq!(config.iter().collect::<Vec<_>>().len(), 4);
        for word in config.iter() {
            assert!(config.vocabulary.contains(word));
        }
    }

    #[test]
    fn it_projects_forward() {
        let config = Config::new("a b. c d. a c. b d.".to_string());
        assert_eq!(
            config.project_forward("a").unwrap(),
            &hashset! {"b".to_string(), "c".to_string()}
        );
    }

    #[test]
    fn it_projects_backward() {
        let config = Config::new("a b. c d. a c. b d.".to_string());
        assert_eq!(
            config.project_backward("b").unwrap(),
            &hashset! {"a".to_string()}
        );
    }
}
