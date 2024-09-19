use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};

use crate::direction::Direction;

#[derive(Debug)]
pub struct InputHandler {
    direction: Direction,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            direction: Direction::Right,
        }
    }

    pub fn poll_input(&mut self) -> bool {
        if poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = read().unwrap() {
                self.direction = match key_event.code {
                    KeyCode::Left | KeyCode::Char('a') => Direction::Left,
                    KeyCode::Right | KeyCode::Char('d') => Direction::Right,
                    KeyCode::Up | KeyCode::Char('w') => Direction::Up,
                    KeyCode::Down | KeyCode::Char('s') => Direction::Down,
                    _ => self.direction,
                };

                return true;
            }
        }

        false
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }
}
