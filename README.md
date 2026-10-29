# Rustle

A Rust command-line harness for comparing Wordle guessing strategies against a bundled answer list.
It runs simulated games and prints the number of guesses needed for each answer.

## Run

Install a Rust toolchain with Cargo, then run from the repository root:

```sh
cargo run --release -- --help
cargo run --release -- --implementation naive --max 10
```

`--implementation` selects the strategy. Available values are:

- `naive`
- `allocations`
- `vec-remaining`
- `initialize-once`
- `weight`
- `prune`

`--max` limits how many entries from `answers.txt` are played. Omitting it runs the full answer list.
The simulation allows up to 32 guesses per game, as defined in [src/lib.rs](src/lib.rs).

## Repository layout

| Path | Purpose |
|---|---|
| [src/main.rs](src/main.rs) | CLI arguments and game loop |
| [src/lib.rs](src/lib.rs) | Guess feedback, candidate matching, and tests |
| [src/algorithms/](src/algorithms/) | The six strategy implementations |
| `answers.txt` | Answers used by the CLI harness |
| `dictionary.txt` | Candidate words with frequencies |
| `wordle.txt` | Additional checked-in word list |
| [flamegraph.svg](flamegraph.svg) | Saved profiling output |

## Checks

```sh
cargo build --release
cargo test
```

The release profile retains debug information for profiling.
