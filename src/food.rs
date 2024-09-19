use rand::Rng;

use crate::{
    constants::{BOARD_HEIGHT, BOARD_WIDTH},
    coordinate::Coordinate,
    snake::Snake,
};

#[derive(Debug)]
pub struct Food {
    pub position: Coordinate,
}

impl Food {
    pub fn new(snake: &Snake) -> Self {
        let mut rng = rand::thread_rng();
        let mut position;

        loop {
            position = Coordinate(
                rng.gen_range(1..BOARD_WIDTH - 1),
                rng.gen_range(1..BOARD_HEIGHT - 1),
            );

            if !snake.body().contains(&position) {
                break;
            }
        }

        Self { position }
    }
}
