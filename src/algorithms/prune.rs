use once_cell::sync::OnceCell;
use std::borrow::Cow;

use crate::{Correctness, Guess, Guesser, DICTIONARY};

#[derive(Clone, Copy)]
struct DictionaryEntry {
    word: &'static str,
    count: usize,
}

static INITIAL_DICTIONARY: OnceCell<Vec<DictionaryEntry>> = OnceCell::new();
static PATTERNS: OnceCell<Vec<[Correctness; 5]>> = OnceCell::new();

pub struct Prune {
    remaining: Cow<'static, Vec<DictionaryEntry>>,
    patterns: Cow<'static, Vec<[Correctness; 5]>>,
}

impl Prune {
    pub fn new() -> Self {
        let remaining = Cow::Borrowed(INITIAL_DICTIONARY.get_or_init(|| {
            Vec::from_iter(DICTIONARY.lines().map(|line| {
                let (word, count) = line
                    .split_once(' ')
                    .expect("Every line consists of `word frequency`");

                let count: usize = count
                    .parse()
                    .expect("Every count must be an unsigned integer");

                DictionaryEntry { word, count }
            }))
        }));

        let patterns = Cow::Borrowed(PATTERNS.get_or_init(|| Correctness::patterns().collect()));

        Prune {
            remaining,
            patterns,
        }
    }
}

impl Default for Prune {
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

impl Guesser for Prune {
    fn guess(&mut self, history: &[Guess]) -> String {
        if let Some(last) = history.last() {
            if matches!(self.remaining, Cow::Owned(_)) {
                self.remaining
                    .to_mut()
                    .retain(|entry| last.matches(entry.word))
            } else {
                self.remaining = Cow::Owned(
                    self.remaining
                        .iter()
                        .filter(|entry| last.matches(entry.word))
                        .copied()
                        .collect(),
                )
            }
        }

        if history.is_empty() {
            self.patterns = Cow::Borrowed(
                PATTERNS
                    .get()
                    .expect("Patterns should already have been constructed."),
            );
            return "tares".to_string();
        } else {
            assert!(!self.patterns.is_empty())
        }

        let remaining_count: usize = self.remaining.iter().map(|entry| entry.count).sum();

        let mut best: Option<Candidate> = None;

        for entry in &*self.remaining {
            let word = entry.word;
            let mut sum = 0.0;

            // We guessed `word` and got `pattern` as correctness.
            // Now, compute what **then** is `left_in_pattern`.
            for pattern in Correctness::patterns() {
                let mut in_pattern_total = 0;

                for entry in &*self.remaining {
                    let guess = Guess {
                        word: Cow::Borrowed(word),
                        mask: pattern,
                    };

                    if guess.matches(entry.word) {
                        in_pattern_total += entry.count;
                    }
                }

                // Source: https://en.wikipedia.org/wiki/Information_theory#Entropy_of_an_information_source
                if in_pattern_total == 0 {
                    continue;
                }
                let probability_of_pattern = in_pattern_total as f64 / remaining_count as f64;
                sum += probability_of_pattern * probability_of_pattern.log2();
            }

            let probability_of_word = entry.count as f64 / remaining_count as f64;
            let score = probability_of_word * (-sum);

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
