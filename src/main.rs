mod ai;
mod constants;
mod coordinate;
mod direction;
mod food;
mod game;
mod input;
mod snake;

use std::io::{self, Write};

use game::Game;

fn main() {
    println!("Select Difficulty Level:");
    println!("1. Easy");
    println!("2. Medium");
    println!("3. Hard");
    print!("Enter your choice (1-3): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim().parse::<u32>().unwrap_or(2);

    let (game_speed, autopilot) = match choice {
        1 => (150, false),
        2 => (100, false),
        3 => (50, false),
        4 => (100, true),
        _ => (100, false),
    };

    let mut game = Game::new(game_speed, autopilot);
    game.run();
}
