use rustle::{algorithms::Naive, Wordle};

const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let wordle = Wordle::default();

    for answer in GAMES.split_whitespace() {
        let guesser = Naive::new();
        if let Some(score) = wordle.play(answer, guesser) {
            println!("guessed {} in {} turns", answer, score);
        } else {
            eprintln!("Failed to guess!");
        }
    }
}
