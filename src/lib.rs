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

struct Config<'a> {
    vocabulary: HashSet<String>,
    forward: HashMap<&'a str, HashSet<&'a str>>,
    backward: HashMap<&'a str, HashSet<&'a str>>,
}

fn add_sentence<'a>(config: &'a mut Config, sentence: Vec<String>) {
    for index in 0..sentence.len() - 1 {
       add_entry(config, sentence[index].clone(), sentence[index + 1].clone());
    }
}

fn add_entry<'a>(config: &'a mut Config, word: String, next_word: String) {
    add_to_mapping(& mut config.forward, &word, &next_word);
    // add_to_mapping(& mut config.backward, &next_word, &word);
    config.vocabulary.insert(word);
    config.vocabulary.insert(next_word);
}

fn add_to_mapping<'a>(mapping: &'a mut HashMap<&'a str, HashSet<&'a str>>, key: &'a str, value: &'a str) {
    mapping
        .entry(key)
        .or_insert(HashSet::new())
        .insert(value);
}