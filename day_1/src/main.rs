use std::fs;

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("input.txt");
}
