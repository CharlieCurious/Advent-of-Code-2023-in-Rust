use std::env;

use day_3::common::parser::parse;

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Must provide a file path.");
    let lines = file_path
        .lines()
        .enumerate();
    parse(lines);
}
