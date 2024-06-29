use day_1::{part_one, part_two};
use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Did no provide file path argument");
    let file_string = fs::read_to_string(file_path).expect("Should have been able to read file");
    let sum_part_1 = part_one::get_calibrations_sum(file_string.clone());
    let sum_part_2 = file_string.lines()
        .map(|line| part_two::process_line(line)).sum::<u32>();

    println!("The sum is {}", sum_part_1);
    println!("The sum is {}", sum_part_2);
}
