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

fn make_vocabulary(cleaned: &Vec<Vec<String>>) -> HashSet<String> {
    cleaned
        .iter()
        .map(|x| x.iter().cloned().collect())
        .collect::<Vec<HashSet<String>>>()
        .iter()
        .fold(HashSet::new(), |acc, x| acc.union(x).cloned().collect())
}

fn make_forward<'a>(
    cleaned: &'a Vec<Vec<String>>,
    vocabulary: &'a HashSet<String>,
) -> HashMap<&'a str, HashSet<&'a str>> {

    let mut acc = HashMap::new();
    for cur in to_sliding_tuples(cleaned) {
        let (f, s) = cur;
        let ref_f: &str = vocabulary.get(&f).unwrap();
        let ref_s: &str = vocabulary.get(&s).unwrap();
        let seconds = acc.entry(ref_f).or_insert(HashSet::new());
        seconds.insert(ref_s);
    }
    acc
}

struct Config<'a> {
    vocabulary: HashSet<String>,
    forward: HashMap<&'a str, HashSet<&'a str>>,
    backward: HashMap<&'a str, HashSet<&'a str>>,
}

pub fn make_config<'a>(text: String) -> Config<'a> {
    let cleaned: Vec<Vec<String>> = clean_sentences(text);
    let vocabulary: HashSet<String> = make_vocabulary(&cleaned);
    let forward: HashMap<&'a str, HashSet<&'a str>> = make_forward(&cleaned, &vocabulary);
    let backward: HashMap<&'a str, HashSet<&'a str>> = make_forward(&cleaned, &vocabulary);
    Config {
        vocabulary: vocabulary,
        forward: forward,
        backward: backward,
    }
}

fn make_backward<'a>(
    cleaned: &'a Vec<Vec<String>>,
    vocabulary: &'a HashSet<String>,
) -> HashMap<&'a str, HashSet<&'a str>> {

    let mut acc = HashMap::new();
    for cur in to_sliding_tuples(cleaned) {
        let (f, s) = cur;
        let ref_f: &str = vocabulary.get(&f).unwrap();
        let ref_s: &str = vocabulary.get(&s).unwrap();
        let firsts = acc.entry(ref_s).or_insert(HashSet::new());
        firsts.insert(ref_f);
    }
    acc
}

fn to_sliding_tuples(sentences: &Vec<Vec<String>>) -> Vec<(String, String)> {
    sentences
        .iter()
        .map(|sentence| window_tuples(sentence.to_vec()))
        .flatten()
        .collect()
}

fn window_tuples(sentence: Vec<String>) -> Vec<(String, String)> {
    sentence
        .windows(2)
        .map(|pair| (pair[0].clone(), pair[1].clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_produces_a_vector_of_sentences() {
        assert_eq!(
            clean_sentences("A \n\tb ,; ' : c? D   e! F g.".to_string()),
            vec!(
                vec!("a".to_string(), "b".to_string(), "c".to_string()),
                vec!("d".to_string(), "e".to_string()),
                vec!("f".to_string(), "g".to_string())
            )
        );
    }

    #[test]
    fn make_vocabulary_makes_a_set_of_all_words() {
        assert_eq!(
            make_vocabulary(&vec!(
                vec!("a".to_string(), "b".to_string(), "b".to_string()),
                vec!("c".to_string(), "d".to_string()),
                vec!("c".to_string(), "d".to_string())
            )),
            [
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ]
            .iter()
            .cloned()
            .collect()
        )
    }
}
