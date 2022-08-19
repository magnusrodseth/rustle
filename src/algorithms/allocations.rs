use std::{borrow::Cow, collections::HashMap};

use crate::{Correctness, Guess, Guesser, DICTIONARY};

pub struct Allocations {
    remaining: HashMap<&'static str, usize>,
}

impl Allocations {
    pub fn new() -> Self {
        let remaining = HashMap::from_iter(DICTIONARY.lines().map(|line| {
            let (word, count) = line
                .split_once(' ')
                .expect("Every line consists of `word frequency`");

            let count: usize = count
                .parse()
                .expect("Every count must be an unsigned integer");

            (word, count)
        }));

        Allocations { remaining }
    }
}

impl Default for Allocations {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone)]
struct Candidate {
    word: &'static str,
    /// The score measures how good the candidate is. A higher score means a better candidate.
    score: f64,
}

impl Guesser for Allocations {
    fn guess(&mut self, history: &[Guess]) -> String {
        if let Some(last) = history.last() {
            self.remaining.retain(|word, _| last.matches(word))
        }

        if history.is_empty() {
            return "tares".to_string();
        }

        let remaining_count: usize = self.remaining.iter().map(|(_, &count)| count).sum();

        let mut best: Option<Candidate> = None;

        for (&word, &_) in &self.remaining {
            let mut sum = 0.0;

            // We guessed `word` and got `pattern` as correctness.
            // Now, compute what **then** is `left_in_pattern`.
            for pattern in Correctness::patterns() {
                let mut in_pattern_total = 0;

                for (candidate, count) in &self.remaining {
                    let guess = Guess {
                        word: Cow::Borrowed(word),
                        mask: pattern,
                    };

                    if guess.matches(candidate) {
                        in_pattern_total += count;
                    }
                }

                // Source: https://en.wikipedia.org/wiki/Information_theory#Entropy_of_an_information_source
                if in_pattern_total == 0 {
                    continue;
                }
                let probability_of_pattern = in_pattern_total as f64 / remaining_count as f64;
                sum += probability_of_pattern * probability_of_pattern.log2();
            }

            let score = 0.0 - sum;

            if let Some(candidate) = best {
                // Determine if the current candidate is better than the current best
                if score > candidate.score {
                    best = Some(Candidate { word, score })
                }
            } else {
                best = Some(Candidate { word, score })
            }
        }

        // TODO: Handle unwrap here
        best.unwrap().word.to_string()
    }
}
