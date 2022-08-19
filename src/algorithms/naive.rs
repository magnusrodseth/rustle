use crate::{Guess, Guesser};

pub struct Naive;

impl Naive {
    pub fn new() -> Self {
        Naive
    }
}

impl Default for Naive {
    fn default() -> Self {
        Self::new()
    }
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        todo!()
    }
}
