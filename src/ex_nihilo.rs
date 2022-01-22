use crate::{Config, Ortho};
use std::collections::BTreeSet;

pub fn create(config: &Config, a: usize) -> BTreeSet<Ortho> {
    let mut results = BTreeSet::default();
    // a -> b -> d <- c <- a'
    // a == a'
    // b != c

    if let Some(potential_bs) = config.project_forward(a) {
        for b in potential_bs {
            if let Some(potential_ds) = config.project_forward(*b) {
                for d in potential_ds {
                    if let Some(potential_cs) = config.project_backward(*d) {
                        for c in potential_cs {
                            if b != c {
                                if let Some(potential_a_primes) = config.project_backward(*c) {
                                    for a_prime in potential_a_primes {
                                        if a == *a_prime {
                                            results.insert(Ortho::new(a, *b, *c, *d));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    results // todo now add repo set subract here
}

#[cfg(test)]
mod tests {
    use super::*;
    use string_interner::Symbol;
    #[test]
    fn it_can_be_made() {
        let (config, interner) = Config::from_sentences("a b. c d. a c. b d.".to_string());
        let res = create(&config, interner.get("a").unwrap().to_usize());
        assert!(res.len() == 1);
    }
}
