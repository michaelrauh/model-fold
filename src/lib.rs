pub fn clean_sentences(sentences: String) -> Vec<Vec<String>> {

    sentences
        .split(&['.', '!', '?'][..])
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            clean_sentences("A \n\tb ,; ' : c? D   e! F g.".to_string()),
            vec!(
                vec!("a".to_string(), "b".to_string(), "c".to_string()),
                vec!("d".to_string(), "e".to_string()),
                vec!("f".to_string(), "g".to_string())
            )
        );
    }
}
