#[derive(Debug, PartialEq, Eq)]
pub struct NumberCoordinates {
    pub row: u32,
    pub col_start: u32,
    pub col_end: u32
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Point {
    row: u32,
    col: u32
}

impl Point {
    pub fn new(row: u32, col: u32) -> Self {
        Point { row, col }
    }
}