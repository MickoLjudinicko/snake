#[cfg(test)]
mod tests {
    use snake::{
        constants::{BOARD_HEIGHT, BOARD_WIDTH},
        coordinate::Coordinate,
        direction::Direction,
        snake::Snake,
    };

    #[test]
    fn test_snake_initialization() {
        let snake = Snake::new();
        assert_eq!(snake.body.len(), 1);
        assert_eq!(snake.body[0], Coordinate(BOARD_WIDTH / 2, BOARD_HEIGHT / 2));
        assert_eq!(snake.direction, Direction::Right);
        assert!(!snake.has_eaten);
    }

    #[test]
    fn test_change_direction() {
        let mut snake = Snake::new();
        snake.change_direction(Direction::Up);
        assert_eq!(snake.direction, Direction::Up);

        // Ensure the snake cannot reverse direction directly
        snake.change_direction(Direction::Down);
        assert_eq!(snake.direction, Direction::Up);

        snake.change_direction(Direction::Left);
        assert_eq!(snake.direction, Direction::Left);

        snake.change_direction(Direction::Right);
        assert_eq!(snake.direction, Direction::Left);
    }

    #[test]
    fn test_move_forward() {
        let mut snake = Snake::new();
        snake.move_forward();
        assert_eq!(snake.body.len(), 1);
        assert_eq!(
            snake.body[0],
            Coordinate(BOARD_WIDTH / 2 + 1, BOARD_HEIGHT / 2)
        );

        snake.change_direction(Direction::Down);
        snake.move_forward();
        assert_eq!(
            snake.body[0],
            Coordinate(BOARD_WIDTH / 2 + 1, BOARD_HEIGHT / 2 + 1)
        );
    }

    #[test]
    fn test_grow() {
        let mut snake = Snake::new();
        snake.grow();
        snake.move_forward();
        assert_eq!(snake.body.len(), 2);
        assert_eq!(
            snake.body[0],
            Coordinate(BOARD_WIDTH / 2 + 1, BOARD_HEIGHT / 2)
        );
        assert_eq!(snake.body[1], Coordinate(BOARD_WIDTH / 2, BOARD_HEIGHT / 2));
    }

    #[test]
    fn test_collides_with_wall() {
        let mut snake = Snake::new();
        for _ in 0..BOARD_WIDTH {
            snake.move_forward();
        }
        assert!(snake.collides_with_wall());
    }

    #[test]
    fn test_body() {
        let snake = Snake::new();
        let body = snake.body();
        assert_eq!(body.len(), 1);
        assert_eq!(body[0], Coordinate(BOARD_WIDTH / 2, BOARD_HEIGHT / 2));
    }
}
