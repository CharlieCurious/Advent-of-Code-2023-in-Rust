use std::{env, fs};

use day_2::part_one;

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Did not provide file path");
    let file_lines: u32 = fs::read_to_string(file_path)
        .expect("Should've been hable to read the file")
        .lines()
        .map(|line| part_one::get_valid_game_ids(line))
        .sum();
    
    
    println!("{}", file_lines)
}
