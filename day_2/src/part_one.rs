use crate::common::{bag::Bag, game::Game, parser};

pub fn get_valid_game_ids(line: &str) -> u32 {
    let game = parser::parse_game_input(line).unwrap();
    let bag = Bag::new(12, 13, 14);
    if is_game_valid(&game, &bag) {
        return game.get_id();
    }
    return 0;
}

pub fn is_game_valid(game: &Game, bag: &Bag) -> bool {
    game.get_permutations().iter()
        .all(|permutation| permutation.is_possible(bag))
}


#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::{common::cube_permutation::CubePermutation, part_one::*};

    #[rstest]
    #[case(Game::new(0, vec![
        CubePermutation::new(4, 0, 3), 
        CubePermutation::new(1, 2, 6),
        CubePermutation::new(0, 2, 0)]), true)]
    #[case(Game::new(1, vec![
        CubePermutation::new(0, 2, 1), 
        CubePermutation::new(1, 3, 4),
        CubePermutation::new(0, 1, 1)]), true)]
    #[case(Game::new(2, vec![
        CubePermutation::new(20, 8, 6), 
        CubePermutation::new(4, 13, 5),
        CubePermutation::new(1, 5, 0)]), false)]
    #[case(Game::new(3, vec![
        CubePermutation::new(3, 1, 6), 
        CubePermutation::new(6, 3, 0),
        CubePermutation::new(14, 3, 15)]), false)]
    #[case(Game::new(4, vec![
        CubePermutation::new(6, 3, 1), 
        CubePermutation::new(1, 2, 2)]), true)]
    fn should_validate_game(#[case] game: Game, #[case] expected: bool) {
        // Arrange
        let bag = Bag::new(12, 13, 14);

        // Act & Assert
        assert_eq!(expected, is_game_valid(&game, &bag));
    }
}