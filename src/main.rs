mod ai;
mod constants;
mod coordinate;
mod direction;
mod food;
mod game;
mod input;
mod music;
mod snake;
mod sound;

use game::{get_difficulty_choice, print_difficulty_selection, Game};

fn main() {
    print_difficulty_selection();

    let (game_speed, autopilot) = get_difficulty_choice();

    let mut game = Game::new(game_speed, autopilot);
    game.run();
}
