use crate::utils::{get_difficulty, get_game_data, select_random_fruit, read_guess};
use std::io::{self, Write};
use colored::*;

pub fn start_game(player_name: &str) {
    loop {
        println!("\n🌟 Hello, {}! Choose difficulty:", player_name);
        let difficulty = get_difficulty();
        let (fruit_list, max_attempts) = get_game_data(difficulty);
        let random_fruit = select_random_fruit(&fruit_list);

        println!("\nGuess the fruit! You have {} attempts.", max_attempts);
        let mut score = 0;

        for attempt in 1..=max_attempts {
            print!("🔢 Attempt {}/{}: ", attempt, max_attempts);
            io::stdout().flush().unwrap();

            let guess = read_guess().to_lowercase();

            if !fruit_list.contains(&guess.as_str()) {
                println!("⚠️ Invalid fruit! Try again.");
                continue;
            }

            if guess == random_fruit {
                println!("🎉 Correct! You guessed it!");
                score += 10;
                break;
                
            } else {
                println!("❌ Incorrect!");
                score -= 5;

                if attempt == 1 {
                    println!(
                        "{}",
                        format!("💡 Hint: The fruit starts with '{}'", &random_fruit[0..1])
                            .yellow()
                            .italic()
                    );
                } else if attempt == max_attempts {
                    println!(
                        "{}",
                        format!("💀 Out of attempts! The correct fruit was '{}'", random_fruit)
                            .bright_red()
                            .bold()
                    );
                }
            }
        }

        println!("{}", format!("🏆 Final score: {} points", score).cyan().bold());

        println!("\n🔁 Do you want to play again? (yes/no)");
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).unwrap();
        if play_again.trim().to_lowercase() != "yes" {
            println!("👋 Goodbye, {}! Thanks for playing!", player_name);
            break;
        }
    }
}
