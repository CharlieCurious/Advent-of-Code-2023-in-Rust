use std::{env, fs};
use day_1::part_one;

fn main() {
    let file_path = env::args().nth(1)
        .expect("Did no provide file path argument");
    let file_string = fs::read_to_string(file_path)
        .expect("Should have been able to read file");
    let sum = part_one::get_calibrations_sum(file_string);

    println!("The sum is {}", sum);
}
