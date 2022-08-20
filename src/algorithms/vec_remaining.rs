use crate::{Correctness, Guess, Guesser, Word, DICTIONARY};

struct DictionaryEntry {
    word: Word,
    count: usize,
}

pub struct VecRemaining {
    remaining: Vec<DictionaryEntry>,
}

impl VecRemaining {
    pub fn new() -> Self {
        let remaining = Vec::from_iter(DICTIONARY.lines().map(|line| {
            let (word, count) = line
                .split_once(' ')
                .expect("Every line consists of `word frequency`");

            let count: usize = count
                .parse()
                .expect("Every count must be an unsigned integer");

            let word = word
                .as_bytes()
                .try_into()
                .expect("Every word should be of length 5.");

            DictionaryEntry { word, count }
        }));

        VecRemaining { remaining }
    }
}

impl Default for VecRemaining {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone)]
struct Candidate {
    word: Word,
    /// The score measures how good the candidate is. A higher score means a better candidate.
    score: f64,
}

impl Guesser for VecRemaining {
    fn guess(&mut self, history: &[Guess]) -> Word {
        if let Some(last) = history.last() {
            self.remaining.retain(|entry| last.matches(entry.word))
        }

        if history.is_empty() {
            return *b"tares";
        }

        let remaining_count: usize = self.remaining.iter().map(|entry| entry.count).sum();

        let mut best: Option<Candidate> = None;

        for entry in &self.remaining {
            let word = entry.word;
            let mut sum = 0.0;

            // We guessed `word` and got `pattern` as correctness.
            // Now, compute what **then** is `left_in_pattern`.
            for pattern in Correctness::patterns() {
                let mut in_pattern_total = 0;

                for entry in &self.remaining {
                    let guess = Guess {
                        word,
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
        best.unwrap().word
    }
}
