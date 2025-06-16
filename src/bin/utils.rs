use std::io::{self, Write};                   
use rand::prelude::*;          
use colored::*;

pub fn get_player_name() -> String {
    print!("{}", "📝 Enter your name: ".blue().bold());
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    name.trim().to_string()
}

pub fn get_difficulty() -> u8 {
    println!("1. Easy 🍏");
    println!("2. Medium 🍊");
    println!("3. Hard 🍍");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => return 1,
            "2" => return 2,
            "3" => return 3,
            _ => println!("❗Please enter 1, 2, or 3:"),
        }
    }
}

pub fn get_game_data(difficulty: u8) -> (Vec<&'static str>, u8) {
    match difficulty {
        1 => (vec!["apple", "banana", "grape"], 5),
        2 => (vec!["apple", "banana", "grape", "mango", "orange"], 4),
        3 => (
            vec![
                "pomegranate", "blueberry", "blackberry", "grapefruit", "dragonfruit",
                "papaya", "pear", "plum", "kiwi"
            ],
            3
        ),
        _ => unreachable!(),
    }
}

pub fn select_random_fruit<'a>(fruit_list: &'a [&'a str]) -> &'a str {
    let mut rng = thread_rng();
    let index = rng.gen_range(0..fruit_list.len());
    fruit_list[index]
}


pub fn read_guess() -> String {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    guess.trim().to_lowercase()
}
