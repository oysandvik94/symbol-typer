use console::Term;
use rand::seq::SliceRandom;

fn main() {
    let symbols: Vec<char> = "!@#$%^&*()_+-=~\"{}'|;:,.<>?/".chars().collect();

    loop {
        let symbol_to_match: char = *symbols.choose(&mut rand::thread_rng()).unwrap();
        play_game(&symbols, symbol_to_match, 0, 0);
    }
}

fn pick_symbol(symbols: &[char]) -> char {
    *symbols.choose(&mut rand::thread_rng()).unwrap()
}

fn play_game(symbols: &[char], symbol_to_match: char, streak: u32, highest_streak: u32) -> u32 {
    println!("Streak: {streak}. Highest streak: {highest_streak}");
    println!("{symbol_to_match}");

    let term = Term::stdout();
    let character: char = term.read_char().expect("Should be a character");

    match test_characters(character, symbol_to_match) {
        RoundResult::Correct => {
            println!("Correct!");
            play_game(symbols, pick_symbol(symbols), streak + 1, std::cmp::max(streak, highest_streak))
        }
        RoundResult::Incorrect { played, target } => {
            println!("Feil!! Du skrev {} men det var {}", played, target);
            play_game(symbols, symbol_to_match, 0, highest_streak)
        }
    }
}

enum RoundResult {
    Correct,
    Incorrect { played: char, target: char },
}
fn test_characters(played: char, target: char) -> RoundResult {
    match played.eq(&target) {
        true => RoundResult::Correct,
        false => RoundResult::Incorrect { played, target },
    }
}
