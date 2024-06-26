use std::{env, fs};

use day_1::get_calibrations_sum;

fn main() {
    let file_path = env::args().nth(1)
    .expect("Did no provide file path argument");
    let file_string = fs::read_to_string(file_path)
        .expect("Should have been able to read file");
    let sum = get_calibrations_sum(file_string);

    println!("The sum is {}", sum);
}
