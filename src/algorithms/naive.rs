use std::collections::HashMap;

use crate::{Guess, Guesser, DICTIONARY};

pub struct Naive {
    remaining: HashMap<&'static str, usize>,
}

impl Naive {
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

        Naive { remaining }
    }
}

impl Default for Naive {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone)]
struct Candidate {
    word: &'static str,
    count: usize,
    score: f64,
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        if let Some(last) = history.last() {
            self.remaining.retain(|word, _| last.matches(word))
        }

        let mut best: Option<Candidate> = None;

        for (&word, &count) in &self.remaining {
            // TODO: Compute this
            let score: f64 = 0.0;

            if let Some(candidate) = best {
                // Determine if the current candidate is better than the current best
                if score > candidate.score {
                    best = Some(Candidate { word, count, score })
                }
            } else {
                best = Some(Candidate { word, count, score })
            }
        }

        // TODO: Handle unwrap here
        best.unwrap().word.to_string()
    }
}
