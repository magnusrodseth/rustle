use std::{borrow::Cow, collections::HashSet};

pub mod algorithms;

const WORD_LENGTH: usize = 5;
const DICTIONARY: &str = include_str!("../dictionary.txt");

pub struct Wordle {
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        let dictionary = HashSet::from_iter(DICTIONARY.lines().map(|line| {
            line.split_once(' ')
                .expect("Every line consists of `word frequency`")
                .0
        }));
        Self { dictionary }
    }

    pub fn play<G: Guesser>(&self, answer: &'static str, mut guesser: G) -> Option<usize> {
        let mut history = Vec::new();

        for i in 1..=32 {
            let guess = guesser.guess(&history[..]);

            if guess == answer {
                return Some(i);
            }

            assert!(self.dictionary.contains(&*guess));

            let correctness = Correctness::compute(answer, &guess);
            history.push(Guess {
                word: Cow::Owned(guess),
                mask: correctness,
            });
        }

        None
    }
}

impl Default for Wordle {
    fn default() -> Self {
        Self::new()
    }
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
    /// Computes the correctness of a guess.
    ///
    /// ## Parameters
    ///
    /// answer - is the answer to check the guess against
    /// guess - is the provided guess to check
    ///
    /// ## Returns
    ///
    /// An array of `Correctness` with fized size 5.
    /// This resembles what parts of the guess was `Correct`, `Misplaced`, or `Wrong`.
    ///
    /// ## Examples
    ///
    /// `Correctness::compute("abcde", "abcde")` => `[Correctness::Correct; 5]`
    /// `Correctness::compute("01234", "abcde")` => `[Correctness::Wrong; 5]`
    /// `Correctness::compute("abcde", "eabcd")` => `[Correctness::Misplaced; 5]`
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

    pub fn patterns() -> impl Iterator<Item = [Self; 5]> {
        itertools::iproduct!(
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong]
        )
        .map(|(a, b, c, d, e)| [a, b, c, d, e])
    }
}

pub struct Guess<'a> {
    word: Cow<'a, str>,
    mask: [Correctness; WORD_LENGTH],
}

impl Guess<'_> {
    fn matches(&self, word: &str) -> bool {
        // If guess G gives mask C against answer A,
        // then guess A should also give mask C against answer G.
        Correctness::compute(word, &self.word) == self.mask
    }
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

impl Guesser for fn(history: &[Guess]) -> String {
    fn guess(&mut self, history: &[Guess]) -> String {
        (*self)(history)
    }
}

#[cfg(test)]
mod tests {
    /// mask![C C C C C] ==> All computed "correctnesses" are correct
    macro_rules! mask {
            (C) => {$crate::Correctness::Correct};
            (M) => {$crate::Correctness::Misplaced};
            (W) => {$crate::Correctness::Wrong};
            ($($c:tt)+) => {[
                $(mask!($c)),+
            ]}
        }

    mod guess_matcher {
        use std::borrow::Cow;

        macro_rules! check {
            // Allows
            ($previous:literal + [$($mask:tt)+] allows $next:literal) => {
                assert!($crate::Guess {
                    word: Cow::Borrowed($previous),
                    mask: mask![$($mask )+],
                }
                .matches($next));

                assert_eq!($crate::Correctness::compute($next, $previous), mask![$($mask )+]);
            };

            // Disallows
            ($previous:literal + [$($mask:tt)+] disallows $next:literal) => {
                assert!(!($crate::Guess {
                    word: Cow::Borrowed($previous),
                    mask: mask![$($mask )+],
                }
                .matches($next)));

                assert_ne!($crate::Correctness::compute($next, $previous), mask![$($mask )+]);
            };
        }

        #[test]
        fn matches() {
            check!("abcde" + [C C C C C] allows "abcde");
            check!("abcdf" + [C C C C C] disallows "abcde");
            check!("abcde" + [W W W W W] allows "01234");
            check!("abcde" + [M M M M M] allows "eabcd");
            check!("aaabb" + [C M W W W] disallows "accaa");
            check!("baaaa" + [W C M W W] allows "aaccc");
            check!("baaaa" + [W C M W W] disallows "caacc");
            check!("tares" + [W M M W W] disallows "brink");
        }
    }
    mod wordle {
        use crate::{Guess, Wordle};

        macro_rules! guesser {
            (|$history:ident| $impl:block) => {{
                struct G;
                impl $crate::Guesser for G {
                    fn guess(&mut self, $history: &[Guess]) -> String {
                        $impl
                    }
                }
                G
            }};
        }

        #[test]
        fn genius() {
            let wordle = Wordle::default();

            let guesser = guesser!(|_history| { "abcde".to_string() });
            let guess_iteration = wordle.play("abcde", guesser);

            assert_eq!(guess_iteration, Some(1));
        }

        #[test]
        fn magnificent() {
            let wordle = Wordle::default();

            let guesser = guesser!(|history| {
                if history.len() == 1 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });

            let guess_iteration = wordle.play("right", guesser);
            assert_eq!(guess_iteration, Some(2));
        }

        #[test]
        fn impressive() {
            let wordle = Wordle::default();

            let guesser = guesser!(|history| {
                if history.len() == 2 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });

            let guess_iteration = wordle.play("right", guesser);
            assert_eq!(guess_iteration, Some(3));
        }
        #[test]
        fn splendid() {
            let wordle = Wordle::default();

            let guesser = guesser!(|history| {
                if history.len() == 3 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });

            let guess_iteration = wordle.play("right", guesser);
            assert_eq!(guess_iteration, Some(4));
        }

        #[test]
        fn great() {
            let wordle = Wordle::default();

            let guesser = guesser!(|history| {
                if history.len() == 4 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });

            let guess_iteration = wordle.play("right", guesser);
            assert_eq!(guess_iteration, Some(5));
        }

        #[test]
        fn phew() {
            let wordle = Wordle::default();

            let guesser = guesser!(|history| {
                if history.len() == 5 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });

            let guess_iteration = wordle.play("right", guesser);
            assert_eq!(guess_iteration, Some(6));
        }

        #[test]
        fn oops() {
            let wordle = Wordle::default();

            let guesser = guesser!(|_history| { "wrong".to_string() });

            let guess_iteration = wordle.play("right", guesser);
            assert_eq!(guess_iteration, None);
        }
    }

    mod compute {
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
