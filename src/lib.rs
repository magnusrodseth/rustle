pub mod algorithms;

const WORD_LENGTH: usize = 5;

pub fn play<G: Guesser>(answer: &'static str, mut guesser: G) -> Option<usize> {
    let mut history = Vec::new();

    for i in 1..=32 {
        let guess = guesser.guess(&history[..]);

        if guess == answer {
            return Some(i);
        }

        let correctness = Correctness::compute(answer, &guess);
        history.push(Guess {
            word: guess,
            mask: correctness,
        });
    }

    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    /// Green
    Correct,
    /// Yellow
    Misplaced,
    /// Gray
    Wrong,
}

impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut correctness = [Correctness::Wrong; 5];

        // Mark as correct
        for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                correctness[i] = Correctness::Correct;
            }
        }

        let mut used = [false; 5];
        for (i, &c) in correctness.iter().enumerate() {
            if c == Correctness::Correct {
                used[i] = true;
            }
        }

        // Mark as misplaced
        for (i, g) in guess.chars().enumerate() {
            if correctness[i] == Correctness::Correct {
                // Already marked as green
                continue;
            }

            if answer.chars().enumerate().any(|(i, a)| {
                if a == g && !used[i] {
                    used[i] = true;
                    return true;
                }
                false
            }) {
                correctness[i] = Correctness::Misplaced;
            }
        }

        correctness
    }
}

pub struct Guess {
    word: String,
    mask: [Correctness; WORD_LENGTH],
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

#[cfg(test)]
mod tests {
    mod compute {
        macro_rules! mask {
            (C) => {Correctness::Correct};
            (M) => {Correctness::Misplaced};
            (W) => {Correctness::Wrong};
            ($($c:tt)+) => {[
                $(mask!($c)),+
            ]}
        }

        use crate::Correctness;
        #[test]
        fn all_correct() {
            assert_eq!(Correctness::compute("abcde", "abcde"), mask![C C C C C]);
        }

        #[test]
        fn all_wrong() {
            assert_eq!(Correctness::compute("01234", "abcde"), mask!(W W W W W))
        }

        #[test]
        fn all_misplaced() {
            assert_eq!(Correctness::compute("abcde", "eabcd"), mask!(M M M M M))
        }

        #[test]
        fn repeat_correct() {
            assert_eq!(Correctness::compute("aabbb", "aaccc"), mask!(C C W W W))
        }

        #[test]
        fn repeat_misplaced() {
            assert_eq!(Correctness::compute("aabbb", "ccaac"), mask!(W W M M W))
        }

        #[test]
        fn repeat_some_green() {
            assert_eq!(Correctness::compute("aabbb", "caacc"), mask!(W C M W W))
        }
    }
}
