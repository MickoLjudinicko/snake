use crate::constants::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::coordinate::Coordinate;
use crate::food::Food;
use crate::input::InputHandler;
use crate::snake::Snake;
use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{stdout, Write};
use std::{thread, time};

pub struct Game {
    snake: Snake,
    food: Food,
    input_handler: InputHandler,
    score: u32,
    game_speed: u64,
    is_autopilot_on: bool,
}

impl Game {
    pub fn new(game_speed: u64, is_autopilot_on: bool) -> Self {
        let snake = Snake::new();
        let food = Food::new(&snake);
        Self {
            snake,
            food,
            input_handler: InputHandler::new(),
            score: 0,
            game_speed,
            is_autopilot_on,
        }
    }

    pub fn run(&mut self) {
        enable_raw_mode().unwrap();
        let mut stdout = stdout();
        execute!(stdout, Hide).unwrap();

        loop {
            if self.input_handler.poll_input() {
                let direction = self.input_handler.get_direction();
                self.snake.change_direction(direction);
            }

            self.snake.move_forward();

            if self.snake.head_position() == self.food.position {
                self.snake.grow();
                self.food = Food::new(&self.snake);
                self.score += 1;
            }

            if self.snake.collides_with_self() || self.snake.collides_with_wall() {
                break;
            }

            self.render();
            thread::sleep(time::Duration::from_millis(self.game_speed));
        }

        self.print_game_over_screen(stdout);
    }

    /// Clears entire screen, moves cursor to the top left corner and prints out game score.
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
                    symbol = 'O';
                    background_color = Color::Green;
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
