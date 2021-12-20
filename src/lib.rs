use std::collections::HashMap;
use std::collections::HashSet;
use std::slice::Windows;

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

fn make_vocabulary(sentences: String) -> HashSet<String> {
    clean_sentences(sentences)
        .iter()
        .map(|x| x.iter().cloned().collect())
        .collect::<Vec<HashSet<String>>>()
        .iter()
        .fold(HashSet::new(), |acc, x| acc.union(x).cloned().collect())
}

fn make_forward<'a>(sentences: String) -> HashMap<&'a str, HashSet<&'a str>> {
    let other_sentences = sentences.clone();
    let cleaned = clean_sentences(sentences);
    let vocabulary = make_vocabulary(other_sentences);

    let better = to_sliding_tuples(cleaned);
    HashMap::new()
}

fn to_sliding_tuples(sentences: Vec<Vec<String>>) -> Vec<(String, String)> {
    sentences.iter().map(|sentence| { window_tuples(sentence.to_vec()) }).flatten().collect()
}

fn window_tuples(sentence: Vec<String>) -> Vec<(String, String)> {
    sentence.windows(2).map(|pair| (pair[0].clone(), pair[1].clone())).collect()
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
            make_vocabulary("a b. c d. b c.".to_string()),
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
