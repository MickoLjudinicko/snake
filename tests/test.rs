use snake::game_logic::gameboard::Gameboard;

#[cfg(test)]
mod tests {
    use snake::game_logic::gameboard::print_gameboard;
    use snake::models::direction::Direction;
    use snake::models::snake::Snake;

    #[test]
    fn test_clone_for_snakes_work() {
        let alter = Snake {
            name: "Snaky",
            direction: Direction::Up,
        };

        assert_eq!(alter.clone(), Snake { ..alter });
    }

    #[test]
    fn print_out_gamefield() {
        print_gameboard(12, 12, '|', '=');
    }

    #[test]
    fn test_assert_gameboard_size() {}
}
