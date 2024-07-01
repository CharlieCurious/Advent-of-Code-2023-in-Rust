use std::num::ParseIntError;
use thiserror::{self, Error};

use super::{cube_permutation::CubePermutation, game::Game};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Missing game name")]
    MissingGameName,
    #[error("Missing game id")]
    MissingGameId,
    #[error("Missing permutations")]
    MissingPermutations,
    #[error("Missing color count")]
    MissingColorName,
    #[error("Invalid number parsing")]
    InvalidNumberParsing(#[from] ParseIntError),
    #[error("Uknown color: {0}")]
    UnknownColor(String)
}

pub fn parse_game_input(line: &str) -> Result<Game, ParseError> {
    let mut split = line.split(":");
    let game_id = split.next()
        .ok_or(ParseError::MissingGameName)?
        .split_whitespace()
        .nth(1)
        .ok_or(ParseError::MissingGameId)?
        .parse::<u32>()?;
    let permutations = split.next()
        .ok_or(ParseError::MissingPermutations)?
        .split(";")
        .map(parse_cube_permutation)
        .collect::<Result<Vec<CubePermutation>, ParseError>>()?;

    Ok(Game::new(game_id, permutations))
}

fn parse_cube_permutation(permutation_string: &str) -> Result<CubePermutation, ParseError> {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;

    for color in permutation_string.split(',') {
        let split: Vec<&str> = color.trim().split_whitespace().collect();
        if split[1].contains("red") {
            red = split[0].parse().unwrap();

        } else if split[1].contains("blue") {
            blue = split[0].parse().unwrap();
        } else {
            green = split[0].parse().unwrap();
        }
    }

    Ok(CubePermutation::new(red, green, blue))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::common::{cube_permutation::CubePermutation, game::Game};

    use super::parse_game_input;


    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        Game::new(1, vec![
            CubePermutation::new(4, 0, 3),
            CubePermutation::new(1, 2, 6),
            CubePermutation::new(0, 2, 0)]))]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        Game::new(2, vec![
            CubePermutation::new(0, 2, 1),
            CubePermutation::new(1, 3, 4),
            CubePermutation::new(0, 1, 1)]))]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        Game::new(3, vec![
            CubePermutation::new(20, 8, 6),
            CubePermutation::new(4, 13, 5),
            CubePermutation::new(1, 5, 0)]))]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        Game::new(4, vec![
            CubePermutation::new(3, 1, 6),
            CubePermutation::new(6, 3, 0),
            CubePermutation::new(14, 3, 15)]))]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        Game::new(5, vec![
            CubePermutation::new(6, 3, 1),
            CubePermutation::new(1, 2, 2)]))]
    fn should_parse_game(#[case] input: &str, #[case] expected: Game) {
        // Act
        let result = parse_game_input(input).unwrap();

        // Assert
        assert_eq!(expected, result);
    }
}