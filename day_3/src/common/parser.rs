use std::{collections::HashSet, iter::Enumerate, str::Lines};

use crate::common::coordinates::Point;

use super::schematic_number::SchematicNumber;

pub fn parse(enumerated_lines: Enumerate<Lines>) {
    let mut found_numbers: Vec<SchematicNumber> = vec![];
    let mut found_chars = HashSet::new();
    for (row, line_string) in enumerated_lines {
        let mut stashed_number: Option<SchematicNumber> = None;
        for (col, ch) in line_string.chars().enumerate() {
            if let Some(number) = ch.to_digit(10) {
                let schematic_number = SchematicNumber::new(number, row as u32, col as u32, col as u32);
                if let Some(stashed) = &mut stashed_number {
                    stashed.merge(schematic_number);
                } else {
                    stashed_number = Some(schematic_number);
                }
            } else {
                if let Some(number) = stashed_number.take() {
                    found_numbers.push(number)
                }
                if ch != '.' {
                    let point = Point::new(row as u32, col as u32);
                    found_chars.insert(point);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::common::parser::parse;

    #[rstest]
    fn test() {
        let test_string = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        parse(test_string.lines().enumerate())
    }
}