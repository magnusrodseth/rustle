use rustle::{algorithms::Naive, play};

const GAMES: &str = include_str!("../answers.txt");

fn main() {
    for answer in GAMES.split_whitespace() {
        let guesser = Naive::new();
        play(answer, guesser);
    }
}
