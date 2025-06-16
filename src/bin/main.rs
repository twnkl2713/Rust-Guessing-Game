mod game;
mod utils;

use game::start_game;
use utils::get_player_name;
use colored::*;

fn main() {
    println!("{}", "🎮 Welcome to the Fruit Guessing Game!".purple().bold());
    let player_name = get_player_name();
    start_game(&player_name);
}