use clap::{ArgEnum, Parser};
use rustle::{
    algorithms::{Allocations, Naive, VecRemaining},
    Guesser, Wordle,
};

const GAMES: &str = include_str!("../answers.txt");

/// TODO
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// TODO
    #[clap(short, long, arg_enum)]
    implementation: Implementation,

    /// TODO
    #[clap(short, long)]
    max: Option<usize>,
}

#[derive(ArgEnum, Debug, Clone, Copy)]
enum Implementation {
    Naive,
    Allocations,
    VecRemaining,
}

fn main() {
    let args = Args::parse();

    match args.implementation {
        Implementation::Naive => play(Naive::new, args.max),
        Implementation::Allocations => play(Allocations::new, args.max),
        Implementation::VecRemaining => play(VecRemaining::new, args.max),
    }
}

/// Plays Wordle based on a provided guesser.
fn play<G>(mut maker: impl FnMut() -> G, max: Option<usize>)
where
    G: Guesser,
{
    let wordle = Wordle::default();
    let games_to_play = GAMES.split_whitespace().take(max.unwrap_or(usize::MAX));

    for answer in games_to_play {
        let guesser = (maker)();
        if let Some(score) = wordle.play(answer, guesser) {
            println!("Guessed '{}' in {} turns.", answer, score);
        } else {
            eprintln!("Failed to guess!");
        }
    }
}
