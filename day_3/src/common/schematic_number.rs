use std::collections::HashSet;

use super::coordinates::{NumberCoordinates, Point};

#[derive(Debug, PartialEq, Eq)]
pub struct SchematicNumber {
    pub value: u32,
    pub coordinates: NumberCoordinates,
    pub adjacent_points: HashSet<Point>
}

impl SchematicNumber {

    pub fn new(value: u32, row: u32, col_start: u32, col_end: u32) -> Self {
        if value > 999 {
            panic!("Numbers should all have between 1 and 3 digits")
        }

        let adjacent_points = SchematicNumber::get_adjacent_points(row, col_start, col_end);

        SchematicNumber {
            value,
            coordinates: NumberCoordinates { row, col_start, col_end },
            adjacent_points
        }
    }
    
    pub fn get_adjacent_points(row: u32, col_start: u32, col_end: u32) -> HashSet<Point> {
        let mut set = HashSet::with_capacity(15);

        let mut insert_if_valid = |r: i32, c: i32| {
            if r >= 0 && c >= 0 {
                set.insert(Point::new(r as u32, c as u32));
            }
        };

        for dr in [-1, 0, 1].into_iter() {
            for dc in [-1, 0, 1].into_iter() {
                if !(dr == 0 && dc == 0) {
                    insert_if_valid(row as i32 + dr, col_start as i32 + dc);
                    insert_if_valid(row as i32 + dr, col_end as i32 + dc);
                }
            }
        }
        return set;
    }
    
    pub fn merge(&mut self, other: SchematicNumber) {
        if other.coordinates.row != self.coordinates.row {
            panic!("Every digit of a number should be on the same row.")
        }

        if other.coordinates.col_start <= self.coordinates.col_end {
            panic!("Other number's digits cannot overlap this number's digits.")
        }
        self.value = self.value * 10 + other.value;
        self.adjacent_points.extend(other.adjacent_points);
        self.coordinates.col_end = other.coordinates.col_end;
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::common::{coordinates::Point, schematic_number::SchematicNumber};
    use std::collections::HashSet;

    #[rstest]
    #[should_panic(expected = "Numbers should all have between 1 and 3 digits")]
    fn should_panic_if_value_has_more_than_3_digits() {
        // Act && Assert
        SchematicNumber::new(1000, 0, 0, 3);
    }

    #[rstest]
    #[case(2, 3, 3, vec![
        Point::new(1, 2), 
        Point::new(1, 3), 
        Point::new(1, 4), 
        Point::new(1, 5), 
        Point::new(1, 6),
        Point::new(2, 2), 
        Point::new(2, 6),
        Point::new(3, 2), 
        Point::new(3, 3), 
        Point::new(3, 4), 
        Point::new(3, 5), 
        Point::new(3, 6)
    ])]
    #[case(0, 1, 1, vec![
        Point::new(0, 0),
        Point::new(1, 0), 
        Point::new(1, 2), 
        Point::new(1, 2)
    ])]
    #[case(1, 1, 1, vec![
        Point::new(0, 0), Point::new(0, 1), Point::new(0, 2),
        Point::new(1, 0), Point::new(1, 2),
        Point::new(2, 0), Point::new(2, 1), Point::new(2, 2)
    ])]
    fn should_get_adjacent_points(
        #[case] row: u32, 
        #[case] col_start: u32, 
        #[case] col_end: u32, 
        #[case] expected_points: Vec<Point>
    ) {

        let result = SchematicNumber::get_adjacent_points(row, col_start, col_end);
        let expected_set: HashSet<Point> = expected_points.into_iter().collect();
        assert_eq!(result, expected_set);
    }

    #[rstest]
    fn should_merge_the_two_numbers() {
        // Arrange
        let mut this_number = SchematicNumber::new(32, 0, 0, 1);
        let other_number = SchematicNumber::new(2, 0, 2, 2);
        let expected_number = SchematicNumber::new(322, 0, 0, 2);

        // Act 
        this_number.merge(other_number);

        // Assert
        assert_eq!(expected_number, this_number);
    }

    #[rstest]
    #[should_panic(expected = "Every digit of a number should be on the same row.")]
    fn should_panic_if_other_number_not_in_the_same_row() {
        // Arrange
        let mut this_number = SchematicNumber::new(32, 0, 0, 1);
        let other_number = SchematicNumber::new(2, 1, 2, 2);

        // Act && Assert
        this_number.merge(other_number);
    }

    #[rstest]
    #[should_panic(expected = "Other number's digits cannot overlap this number's digits.")]
    fn should_panic_if_other_number_digits_overlap_with_this_number() {
        // Arrange
        let mut this_number = SchematicNumber::new(32, 0, 0, 1);
        let other_number = SchematicNumber::new(69, 0, 1, 2);

        // Act && Assert
        this_number.merge(other_number);
    }
}