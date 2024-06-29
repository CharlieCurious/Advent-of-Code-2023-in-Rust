pub fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len())
        .filter_map(|index| {
            let reduced_line = &line[index..];
            let result = if reduced_line.starts_with('1') || reduced_line.starts_with("one") {
                '1'
            } else if reduced_line.starts_with('2') || reduced_line.starts_with("two") {
                '2'
            } else if reduced_line.starts_with('3') || reduced_line.starts_with("three") {
                '3'
            } else if reduced_line.starts_with('4') || reduced_line.starts_with("four") {
                '4'
            } else if reduced_line.starts_with('5') || reduced_line.starts_with("five") {
                '5'
            } else if reduced_line.starts_with('6') || reduced_line.starts_with("six") {
                '6'
            } else if reduced_line.starts_with('7') || reduced_line.starts_with("seven") {
                '7'
            } else if reduced_line.starts_with('8') || reduced_line.starts_with("eight") {
                '8'
            } else if reduced_line.starts_with('9') || reduced_line.starts_with("nine") {
                '9'
            } else {
                reduced_line.chars().next().unwrap()
            };
            result.to_digit(10)
        });
        let first = it.next().expect("Should be a number");

        match it.last() {
            Some(num) => format!("{first}{num}"),
            None => format!("{first}{first}")
        }
        .parse::<u32>()
        .expect("Should be a valid number")
}

#[cfg(test)]
mod tests {
    use crate::part_two::*;
    use rstest::*;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn should_process_lines_correctly(#[case] line: &str, #[case] expected: u32) {
        // act
        let result = process_line(line);

        // assert
        assert_eq!(expected, result)
    }
}
