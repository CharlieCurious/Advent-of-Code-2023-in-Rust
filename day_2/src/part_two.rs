use crate::common::{bag::Bag, game::Game};

pub fn get_minumum_set(game: Game) -> Bag {
    let (reds, greens, blues) = game.get_permutations()
        .into_iter()
        .fold((0, 0, 0), |(reds, greens, blues), permutation| {
            (reds.max(permutation.red), greens.max(permutation.green), blues.max(permutation.blue))
        });
    Bag::new(reds, greens, blues)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::common::{bag::Bag, cube_permutation::CubePermutation, game::Game};
    use super::get_minumum_set;


    #[rstest]
    #[case(Game::new(0, vec![
        CubePermutation::new(4, 0, 3), 
        CubePermutation::new(1, 2, 6),
        CubePermutation::new(0, 2, 0)]), Bag::new(4, 2, 6))]
    #[case(Game::new(1, vec![
        CubePermutation::new(0, 2, 1), 
        CubePermutation::new(1, 3, 4),
        CubePermutation::new(0, 1, 1)]), Bag::new(1, 3, 4))]
    #[case(Game::new(2, vec![
        CubePermutation::new(20, 8, 6), 
        CubePermutation::new(4, 13, 5),
        CubePermutation::new(1, 5, 0)]), Bag::new(20, 13, 6))]
    #[case(Game::new(3, vec![
        CubePermutation::new(3, 1, 6), 
        CubePermutation::new(6, 3, 0),
        CubePermutation::new(14, 3, 15)]), Bag::new(14, 3, 15))]
    #[case(Game::new(4, vec![
        CubePermutation::new(6, 3, 1), 
        CubePermutation::new(1, 2, 2)]), Bag::new(6, 3, 2))]
    fn should_get_minimum_set(#[case] input_game: Game, #[case] expected_bag: Bag) {
        // Act
        let result = get_minumum_set(input_game);

        // Assert
        assert_eq!(expected_bag, result);
    }
}