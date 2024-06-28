pub fn get_calibrations_sum(file_string: String) -> u32 {
    file_string.lines()
        .map(|line| get_calibration_from_line(line))
        .sum()
}

fn get_calibration_from_line(line: &str) -> u32 {
    let line_chars: Vec<char> = line.chars().collect();

    let mut first_index = 0;
    let mut second_index = line.len() - 1;

    let mut first_number: Option<char> = None;
    let mut last_number: Option<char> = None;

    while first_index <= line.len() - 1 {
        if first_number.is_none() {
            let start_ch = line_chars[first_index];
            if start_ch.is_digit(10) {
                first_number = Some(start_ch)
            }
            first_index += 1;
        }

        if last_number.is_none() {
            let end_ch: char = line_chars[second_index];
            if end_ch.is_digit(10) {
                last_number = Some(end_ch)
            }
            if second_index == 0 {
                break;
            }
            second_index -= 1;
        }
        if first_number.is_some() && last_number.is_some() {
            break;
        }

    }
    let first_number = first_number
        .expect("No numbers found on the string. All strings must have at least one number.");
    let last_number = last_number.unwrap_or(first_number);

    combine_digits(first_number, last_number)
}

fn combine_digits(ch1: char, ch2: char) -> u32 {
    let digit1 = ch1.to_digit(10)
        .expect(&format!("{} is not a number", ch1));
    let digit2 = ch2.to_digit(10)
        .expect(&format!("{} is not a number", ch2));

    digit1 * 10 + digit2
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::*;
    
    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("t7rebuchet", 77)]
    #[case("5charlie", 55)]
    #[case("eightg1", 11)]
    fn should_get_calibration_from_line(#[case] input: &str, #[case] expected: u32) {
        // act
        let result = get_calibration_from_line(input);

        // assert
        assert_eq!(expected, result);
    }

    #[test]
    fn should_combine_two_digit_chars_into_u32() {
        // arrange
        let ch1 = '8';
        let ch2 = '4';

        // act
        let result = combine_digits(ch1, ch2);

        // assert
        assert_eq!(84, result);
    }
}