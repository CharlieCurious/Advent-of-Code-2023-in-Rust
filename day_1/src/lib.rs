pub fn get_calibrations_sum(file_string: String) -> u32 {
    file_string.lines()
        .map(|line| get_calibration_from_line(line))
        .sum()
}

fn get_calibration_from_line(line: &str) -> u32 {
    let line_chars: Vec<char> = line.chars().collect();

    let mut start = 0;
    let mut end = line.len() - 1;

    let mut first_number: Option<char> = None;
    let mut last_number: Option<char> = None;

    while start <= line.len()-1 {
        if first_number.is_none() {
            let start_ch = line_chars[start];
            if start_ch.is_digit(10) {
                first_number = Some(start_ch)
            }
            start += 1;
        }

        if last_number.is_none() {
            let end_ch: char = line_chars[end];
            if end_ch.is_digit(10) {
                last_number = Some(end_ch)
            }
            if end == 0 {
                break;
            }
            end -= 1;
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
    use crate::*;
    
    #[test]
    fn should_get_calibration_from_line() {
        // act
        let case0 = get_calibration_from_line("1abc2");
        let case1 = get_calibration_from_line("pqr3stu8vwx");
        let case2 = get_calibration_from_line("a1b2c3d4e5f");
        let case3 = get_calibration_from_line("t7rebuchet");
        let case4 = get_calibration_from_line("5charlie");
        let case5 = get_calibration_from_line("eightg1");

        // assert
        assert_eq!(12, case0);
        assert_eq!(38, case1);
        assert_eq!(15, case2);
        assert_eq!(77, case3);
        assert_eq!(55, case4);
        assert_eq!(11, case5);
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