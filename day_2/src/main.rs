use std::{env, fs};

use day_2::{common::parser::parse_game_input, part_one, part_two::get_minumum_set};

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Did not provide file path");
    let file = fs::read_to_string(file_path)
        .expect("Should've been hable to read the file");
    let part_1 = file.lines().map(|line| part_one::get_valid_game_ids(line))
        .sum::<u32>();
    let part_2: u32 = file.lines()
        .map(|line| {
            let game = parse_game_input(line).unwrap();
            let minimum_set = get_minumum_set(game);
            minimum_set.get_number_of_reds() * minimum_set.get_number_of_greens() * minimum_set.get_number_of_blues()
        }).sum();
    
    
    println!("{}", part_1);
    println!("{}", part_2);
}
