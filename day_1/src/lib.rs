pub fn get_calibrations_sum(file_string: String) -> u32 {
    file_string.lines()
        .map(|line| get_calibration_from_line(line))
        .sum()
}

fn get_calibration_from_line(line: &str) -> u32 {
    let line_chars: Vec<_> = line.chars()
        .collect();

    let mut start = 0;
    let mut end = line.len() - 1;

    let mut first_number: Option<char> = None;
    let mut last_number: Option<char> = None;

    while start <= end {
        if first_number.is_some() && last_number.is_some() {
            break;
        }

        if first_number.is_none() {
            let start_ch = *line_chars.get(start)
                .unwrap();
            if start_ch.is_digit(10) {
                first_number = Some(start_ch)
            }
            start += 1;
        }

        if last_number.is_none() {
            let end_ch: char = *line_chars.get(end)
                .unwrap();
            if end_ch.is_digit(10) {
                last_number = Some(end_ch)
            }
            if end == 0 {
                break;
            }
            end -= 1;
        }
    }

    return process_found_numbers(first_number, last_number);
}

fn process_found_numbers(first_number: Option<char>, last_number: Option<char>) -> u32 {
    match (first_number, last_number) {
        (Some(x), Some(y)) => return combine_digits(x, y),
        (Some(x), None) => combine_digits(x, x),
        (None, Some(y)) => combine_digits(y, y),
        _ => panic!("No number values found on the string. All strings must have at least one number.")
    }
}

fn combine_digits(ch1: char, ch2: char) -> u32 {
    let digit1 = char_to_u32(ch1);
    let digit2 = char_to_u32(ch2);

    digit1 * 10 + digit2
}

fn char_to_u32(ch: char) -> u32 {
    ch.to_digit(10).unwrap_or_else(|| panic!("{} is not a number", ch))
}

#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn should_get_calibration_from_line() {
        // act
        let case0 = get_calibration_from_line("1abc2");
        let case1 = get_calibration_from_line("pqr3stu8vwx");
        let case2 = get_calibration_from_line("a1b2c3d4e5f");
        let case3 = get_calibration_from_line("treb7uchet");

        // assert
        assert_eq!(12, case0);
        assert_eq!(38, case1);
        assert_eq!(15, case2);
        assert_eq!(77, case3);
    }

    #[test]
    fn should_process_found_numbers_correctly() {
        // arrange
        // both have values
        let (x0, y0) = (Some('4'), Some('2'));
        let (x2, y2) = (Some('4'), None);
        let (x3, y3) = (None, Some('4'));

        // act
        let result0 = process_found_numbers(x0, y0);
        let result1 = process_found_numbers(x2, y2);
        let result2 = process_found_numbers(x3, y3);

        // assert
        assert_eq!(42, result0);
        assert_eq!(44, result1);
        assert_eq!(44, result2);
    }

    #[test]
    #[should_panic]
    fn should_panic_if_no_numbers_found() {
        // act && assert
        process_found_numbers(None, None);
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

    #[test]
    fn should_convert_number_char_to_u32() {
        // arrange
        let ch_numbers = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

        // act
        for ch in ch_numbers {
            char_to_u32(ch);
        }
    }

    #[test]
    #[should_panic]
    fn should_panic_if_try_to_convert_non_numerical_char() {
        // arrange
        let non_numerical_char = 'a';

        // act && assert
        char_to_u32(non_numerical_char);
    }
}