use rustle::{algorithms::Naive, Wordle};

const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let wordle = Wordle::default();

    for answer in GAMES.split_whitespace() {
        let guesser = Naive::new();
        wordle.play(answer, guesser);
    }
}
