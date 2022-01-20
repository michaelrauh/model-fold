use crate::ortho::LiteralOrtho;
use crate::repo::Repo;
use crate::Ortho;

pub fn search(input: String) -> Repo {
    Repo::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_finds_atoms() {
        assert_eq!(search("a b. c d. a c. b d.".to_string()).len(), 1);
    }
}
