use console::Term;
use rand::seq::SliceRandom;

fn main() {
    let symbols: Vec<char> = "!@#$%^&*()_+-=~\"{}'|;:,.<>?/".chars().collect();

    let mut streak: u32 = 0;
    let mut highest_streak: u32 = 0;
    loop {
        let symbol_to_match: char = *symbols.choose(&mut rand::thread_rng()).unwrap();

        println!("Streak: {streak}. Highest streak: {highest_streak}");
        println!("{symbol_to_match}");
        
        streak = play_game(symbol_to_match, streak);

        if streak > highest_streak {
            highest_streak = streak;
        }
    }
}

fn play_game(symbol_to_match: char, streak: u32) -> u32 {
    let term = Term::stdout();
    let character: char = term.read_char().expect("Should be a character");

    match test_characters(character, symbol_to_match) {
        RoundResult::Correct => streak + 1,
        RoundResult::Incorrect { played, target } => {
            println!("Feil!! Du skrev {played} men det var {target}");
            println!("{target}");
            play_game(target, 0)
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
