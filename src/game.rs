use crate::constants::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::coordinate::Coordinate;
use crate::direction::Direction;
use crate::food::Food;
use crate::input::InputHandler;
use crate::snake::Snake;
use crate::{music, sound};
use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};

#[derive(Debug, Clone, Copy)]
pub enum GameDifficulty {
    EASY,
    MEDIUM,
    HARD,
}

impl GameDifficulty {
    fn convert_to_number(&self) -> u8 {
        match self {
            GameDifficulty::EASY => 150,
            GameDifficulty::MEDIUM => 100,
            GameDifficulty::HARD => 50,
        }
    }
}

pub struct Game {
    snake: Snake,
    food: Food,
    input_handler: InputHandler,
    score: u32,
    game_difficulty: GameDifficulty,
    is_autopilot_on: bool,
    sound_enabled: bool,
    music_enabled: bool,
}

impl Game {
    pub fn new(
        game_difficulty: GameDifficulty,
        is_autopilot_on: bool,
        sound_enabled: bool,
        music_enabled: bool,
    ) -> Self {
        let snake = Snake::new();
        let food = Food::new(&snake);
        Self {
            snake,
            food,
            input_handler: InputHandler::new(),
            score: 0,
            game_difficulty,
            is_autopilot_on,
            sound_enabled,
            music_enabled,
        }
    }

    pub fn run(&mut self) -> bool {
        enable_raw_mode().unwrap();
        let mut stdout = stdout();
        execute!(stdout, Hide).unwrap();

        // Create a stop signal for the background music thread
        let stop_signal = Arc::new(AtomicBool::new(false));
        let music_stop_signal = stop_signal.clone();

        let theme_notes = music::game_theme();

        // Start the background music thread
        if self.music_enabled {
            thread::spawn(move || {
                music::play_music(theme_notes, music_stop_signal);
            });
        }

        loop {
            if self.is_autopilot_on {
                self.autopilot();
            } else if self.input_handler.poll_input() {
                let direction = self.input_handler.get_direction();
                self.snake.change_direction(direction);
            }

            self.snake.move_forward();

            if self.snake.head_position() == self.food.position {
                self.snake.grow();
                self.food = Food::new(&self.snake);
                self.score += 1;

                if self.sound_enabled {
                    thread::spawn(|| {
                        sound::play_tone(440, 200);
                    });
                }
            }

            if self.snake.collides_with_self() || self.snake.collides_with_wall() {
                if self.sound_enabled {
                    sound::play_tone(220, 500);
                }

                // Signal the music thread to stop
                stop_signal.store(true, Ordering::SeqCst);

                self.print_game_over_screen(stdout);

                return true;
            }

            self.render();
            thread::sleep(time::Duration::from_millis(
                self.game_difficulty.convert_to_number() as u64,
            ));
        }
    }

    fn autopilot(&mut self) {
        let snake_head = self.snake.head_position();
        let food_position = self.food.position;

        // Calculate the direction to food, considering obstacles
        let direction_to_food = self.calculate_direction_to_food(snake_head, food_position);

        // Change the snake's direction
        self.snake.change_direction(direction_to_food);
    }

    fn calculate_direction_to_food(&self, head: Coordinate, food: Coordinate) -> Direction {
        // Helper function to check if the next position in a given direction is safe
        let is_safe = |direction: Direction| -> bool {
            let next_position = match direction {
                Direction::Up => Coordinate(head.0, head.1 - 1),
                Direction::Down => Coordinate(head.0, head.1 + 1),
                Direction::Left => Coordinate(head.0 - 1, head.1),
                Direction::Right => Coordinate(head.0 + 1, head.1),
            };
            !self.snake.body.contains(&next_position)
                && next_position.0 > 0
                && next_position.0 < BOARD_WIDTH - 1
                && next_position.1 > 0
                && next_position.1 < BOARD_HEIGHT - 1
        };

        // Check all directions and choose the best one
        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        let mut best_direction = self.snake.direction;
        let mut min_distance = i32::MAX;

        for &dir in &directions {
            if dir != self.snake.direction.opposite() && is_safe(dir) {
                let next_pos = match dir {
                    Direction::Up => Coordinate(head.0, head.1 - 1),
                    Direction::Down => Coordinate(head.0, head.1 + 1),
                    Direction::Left => Coordinate(head.0 - 1, head.1),
                    Direction::Right => Coordinate(head.0 + 1, head.1),
                };
                let distance = (next_pos.0 - food.0).abs() + (next_pos.1 - food.1).abs();
                if distance < min_distance {
                    min_distance = distance;
                    best_direction = dir;
                }
            }
        }

        best_direction
    }

    /// Clears entire screen, moves cursor to the top left corner, and prints out the game score.
    fn print_game_over_screen(&mut self, mut stdout: std::io::Stdout) {
        execute!(stdout, Show).unwrap();
        clear_screen(&stdout);

        move_cursor_to_top_left_corner(stdout);
        disable_raw_mode().unwrap();
        println!("Game Over! Your score: {}", self.score);
    }

    fn render(&self) {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All)).unwrap();

        let (columns, rows) = crossterm::terminal::size().unwrap();
        let x_offset = (columns as i32 - BOARD_WIDTH) / 2;
        let y_offset = (rows as i32 - BOARD_HEIGHT) / 2;

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let mut symbol = ' ';
                let foreground_color = Color::Reset;
                let mut background_color = Color::Black;

                if x == 0 || x == BOARD_WIDTH - 1 || y == 0 || y == BOARD_HEIGHT - 1 {
                    symbol = ' ';
                    background_color = Color::Blue;
                } else if self.snake.body().contains(&Coordinate(x, y)) {
                    symbol = '@';
                    background_color = Color::Yellow;
                } else if self.food.position == Coordinate(x, y) {
                    symbol = '*';
                    background_color = Color::Red;
                }

                execute!(
                    stdout,
                    MoveTo((x + x_offset) as u16, (y + y_offset) as u16),
                    SetForegroundColor(foreground_color),
                    SetBackgroundColor(background_color),
                    Print(symbol),
                    ResetColor
                )
                .unwrap();
            }
        }

        execute!(
            stdout,
            MoveTo(0, 0),
            SetForegroundColor(Color::Yellow),
            Print(format!("Score: {}", self.score)),
            ResetColor
        )
        .unwrap();

        stdout.flush().unwrap();
    }
}

fn move_cursor_to_top_left_corner(mut stdout: std::io::Stdout) {
    execute!(stdout, MoveTo(0, 0)).unwrap();
}

fn clear_screen(mut stdout: &std::io::Stdout) {
    execute!(stdout, Clear(ClearType::All)).unwrap();
}
