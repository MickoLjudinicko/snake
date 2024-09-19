use crate::{
    constants::{BOARD_HEIGHT, BOARD_WIDTH},
    coordinate::Coordinate,
    direction::Direction,
};

/// Represents the Snake in the classic Snake game, encapsulating its movement,
/// growth, and collision logic. The Snake is composed of a body, represented as a vector
/// of `Coordinate` points, and moves in one of the four cardinal directions.
///
/// # Fields
///
/// - `body`: A vector of `Coordinate` representing the position of each segment of the Snake.
///   The first element is the head of the Snake, and the remaining elements represent its body.
/// - `direction`: The current direction of movement of the Snake, represented as a `Direction` enum.
/// - `has_eaten`: A boolean flag that determines whether the Snake has eaten food and should grow in size.
///
/// # Methods
///
/// - `new() -> Snake`:
///   Creates a new Snake object with an initial size of one segment, located at the center of the game board.
///   The initial direction is set to `Direction::Right`, and `has_eaten` is set to `false`.
///
/// - `change_direction(&mut self, direction: Direction)`:
///   Updates the direction of the Snake, ensuring that the new direction is not directly opposite to the current one.
///   This prevents the Snake from moving into itself, which would result in a collision.
///
/// - `move_forward(&mut self)`:
///   Moves the Snake forward in its current direction by adding a new head at the next position
///   based on the direction. If the Snake has not eaten, its tail is removed to simulate forward motion.
///   If the Snake has eaten, the tail remains, effectively growing the Snake.
///
/// - `head_position(&self) -> Coordinate`:
///   Returns the current position of the Snake's head. The head is always the first segment in the `body` vector.
///
/// - `grow(&mut self)`:
///   Instructs the Snake to grow after it eats food. Sets the `has_eaten` flag to `true`,
///   which will prevent the tail from being removed during the next move, thus growing the Snake.
///
/// - `collides_with_self(&self) -> bool`:
///   Checks if the Snake's head has collided with its own body by comparing the head's position with
///   the positions of the other segments in the body. Returns `true` if a collision occurs, otherwise `false`.
///
/// - `collides_with_wall(&self) -> bool`:
///   Determines whether the Snake's head has collided with the boundary of the game board.
///   Returns `true` if the head moves out of bounds, otherwise `false`.
///
/// - `body(&self) -> &Vec<Coordinate>`:
///   Returns a reference to the vector representing the Snake's body segments. This allows
///   external functions to read the positions of the Snake's segments without modifying them.
///
/// # Example Usage
///
/// ```rust
/// let mut snake = Snake::new();
/// snake.change_direction(Direction::Up);
/// snake.move_forward();
/// if snake.collides_with_self() || snake.collides_with_wall() {
///     println!("Game Over!");
/// }
/// ```
#[derive(Debug)]
pub struct Snake {
    pub body: Vec<Coordinate>,
    pub direction: Direction,
    has_eaten: bool,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: vec![Coordinate(BOARD_WIDTH / 2, BOARD_HEIGHT / 2)],
            direction: Direction::Right,
            has_eaten: false,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        let is_up_and_down_not_simultaneous =
            self.direction == Direction::Up && direction != Direction::Down;

        let is_right_and_left_not_simultaneous =
            self.direction == Direction::Right && direction != Direction::Left;

        let is_left_and_right_not_simultaneous =
            self.direction == Direction::Left && direction != Direction::Right;

        let is_down_and_up_not_simultaneous =
            self.direction == Direction::Down && direction != Direction::Up;

        if is_up_and_down_not_simultaneous
            || is_right_and_left_not_simultaneous
            || is_left_and_right_not_simultaneous
            || is_down_and_up_not_simultaneous
        {
            self.direction = direction;
        }
    }

    pub fn move_forward(&mut self) {
        let coordinate = self.head_position();

        let new_head = match self.direction {
            Direction::Up => Coordinate(coordinate.0, coordinate.1 - 1),
            Direction::Down => Coordinate(coordinate.0, coordinate.1 + 1),
            Direction::Left => Coordinate(coordinate.0 - 1, coordinate.1),
            Direction::Right => Coordinate(coordinate.0 + 1, coordinate.1),
        };

        self.body.insert(0, new_head);

        if !self.has_eaten {
            self.body.pop();
        } else {
            self.has_eaten = false;
        }
    }

    pub fn head_position(&self) -> Coordinate {
        self.body[0]
    }

    pub fn grow(&mut self) {
        // let tail = *self.body.last().unwrap();
        // self.body.push(tail);
        self.has_eaten = true;
    }

    pub fn collides_with_self(&self) -> bool {
        let head = self.head_position();
        self.body.iter().skip(1).any(|&segment| segment == head)
    }

    pub fn collides_with_wall(&self) -> bool {
        let coordinate = self.head_position();
        coordinate.0 <= 0
            || coordinate.0 >= BOARD_WIDTH - 1
            || coordinate.1 <= 0
            || coordinate.1 >= BOARD_HEIGHT - 1
    }

    pub fn body(&self) -> &Vec<Coordinate> {
        &self.body
    }
}
