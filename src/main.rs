mod highscore_store;

use std::{
    env,
    time::{Duration, Instant},
};

use console::Term;
use highscore_store::retrieve_highscore;
use rand::seq::SliceRandom;

use crate::highscore_store::store_highscore;

fn main() {
    let symbols: Vec<char> = "!@#$%^&*()_+-=~\"{}'|;:,.<>?/\\`[]".chars().collect();

    let highest_streak: u32 = match retrieve_highscore() {
        Ok(streak_result) => streak_result,
        Err(error) => {
            eprintln!("Failed to retrieve high score: {error}");
            std::process::exit(1);
        }
    };

    let game: Game = Game {
        symbols,
        max_time: get_max_time(),
    };
    loop {
        game.play_round(game.pick_symbol(), 0, highest_streak);
    }
}

struct Game {
    symbols: Vec<char>,
    max_time: Duration,
}

impl Game {
    fn play_round(&self, symbol_to_match: char, streak: u32, highest_streak: u32) -> u32 {
        println!("Streak: {streak}. Highest streak: {highest_streak}");
        println!("{symbol_to_match}");

        if let Err(error) = store_highscore(highest_streak) {
            eprintln!("Failed to store high score: {error}");
            std::process::exit(1);
        }

        let term = Term::stdout();
        let typed_character = TypedCharacter::type_character(term);

        match self.test_characters(typed_character, symbol_to_match) {
            RoundResult::Correct => {
                println!("Correct!");
                let new_streak = streak + 1;
                self.play_round(
                    self.pick_symbol(),
                    new_streak,
                    std::cmp::max(new_streak, highest_streak),
                )
            }
            RoundResult::Incorrect { played, target } => {
                println!("Feil!! Du skrev {} men det var {}", played, target);
                self.play_round(symbol_to_match, 0, highest_streak)
            }
            RoundResult::TooSlow { used_time } => {
                println!("Too slow! You used {0:?} time", used_time);
                self.play_round(symbol_to_match, 0, highest_streak)
            }
        }
    }

    fn pick_symbol(&self) -> char {
        *self.symbols.choose(&mut rand::thread_rng()).unwrap()
    }

    fn test_characters(&self, played: TypedCharacter, target: char) -> RoundResult {
        match played.character.eq(&target) {
            true => match played.elapsed < self.max_time {
                true => RoundResult::Correct,
                false => RoundResult::TooSlow {
                    used_time: played.elapsed,
                },
            },
            false => RoundResult::Incorrect {
                played: played.character,
                target,
            },
        }
    }
}

struct TypedCharacter {
    character: char,
    elapsed: Duration,
}
impl TypedCharacter {
    fn type_character(term: Term) -> TypedCharacter {
        let start = Instant::now();
        let character: char = term.read_char().expect("Should be a character");
        let elapsed = start.elapsed();

        TypedCharacter { character, elapsed }
    }
}

enum RoundResult {
    Correct,
    TooSlow { used_time: Duration },
    Incorrect { played: char, target: char },
}

// TODO: this method is ugly. should make a general cmd line parser and struct
fn get_max_time() -> Duration {
    println!("{:?}", env::args());
    let mut max_time = Duration::from_secs(1);
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--time" => match args.next() {
                Some(arg_value) => {
                    max_time = match str::parse::<u64>(&arg_value) {
                        Ok(millis) => Duration::from_millis(millis),
                        Err(_) => panic!("{} could not be parsed to milliseconds", arg_value),
                    };
                }
                _ => panic!("--time must be followed by a millisecond value"),
            },
            _ => panic!(
                "{} is an unknown argument. Supporting only --time for now",
                arg
            ),
        };
    }
    max_time
}
