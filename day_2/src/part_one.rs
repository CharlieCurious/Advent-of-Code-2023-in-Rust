use crate::parser;

pub fn get_valid_game_ids(line: &str) -> u32 {
    let game = parser::parse_game_input(line).unwrap();
    let bag = Bag::new(12, 13, 14);
    if game.is_valid(&bag) {
        return game.id;
    }
    return 0;
}

#[derive(Debug, PartialEq)]
pub struct CubePermutation {
    red: u32,
    green: u32,
    blue: u32
}

impl CubePermutation {
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        CubePermutation { red, green, blue }
    }

    pub fn is_possible(&self, bag: &Bag) -> bool {
        return self.red <= bag.reds &&
            self.green <= bag.greens &&
            self.blue <= bag.blues
    }
}

struct Bag {
    reds: u32,
    greens: u32,
    blues: u32
}

impl Bag {
    pub fn new(reds: u32, greens: u32, blues: u32) -> Self {
        Bag { reds, greens, blues }
    }
}

#[derive(Debug, PartialEq)]
pub struct Game {
    id: u32,
    permutations: Vec<CubePermutation>
}

impl Game {
    pub fn new(id: u32, permutations: Vec<CubePermutation>) -> Self {
        Game { id, permutations }
    }

    pub fn is_valid(&self, bag: &Bag) -> bool {
        self.permutations.iter()
            .all(|permutation| permutation.is_possible(bag))
    }
}


#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::part_one::*;

    #[rstest]
    #[case(vec![
        CubePermutation::new(4, 0, 3), 
        CubePermutation::new(1, 2, 6),
        CubePermutation::new(0, 2, 0)], true)]
    #[case(vec![
        CubePermutation::new(0, 2, 1), 
        CubePermutation::new(1, 3, 4),
        CubePermutation::new(0, 1, 1)], true)]
    #[case(vec![
        CubePermutation::new(20, 8, 6), 
        CubePermutation::new(4, 13, 5),
        CubePermutation::new(1, 5, 0)], false)]
    #[case(vec![
        CubePermutation::new(3, 1, 6), 
        CubePermutation::new(6, 3, 0),
        CubePermutation::new(14, 3, 15)], false)]
    #[case(vec![
        CubePermutation::new(6, 3, 1), 
        CubePermutation::new(1, 2, 2)], true)]
    fn should_validate_game(#[case] permutations: Vec<CubePermutation>, #[case] expected: bool) {
        // Arrange
        let game = Game::new(1, permutations);
        let bag = Bag::new(12, 13, 14);

        // Act & Assert
        assert_eq!(expected, game.is_valid(&bag));
    }
}