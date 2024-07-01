#[derive(Debug, PartialEq)]
pub struct Bag {
    reds: u32,
    greens: u32,
    blues: u32
}

impl Bag {
    pub fn new(reds: u32, greens: u32, blues: u32) -> Self {
        Bag { reds, greens, blues }
    }

    pub fn get_number_of_reds(&self) -> u32 {
        self.reds
    }

    pub fn get_number_of_greens(&self) -> u32 {
        self.greens
    }

    pub fn get_number_of_blues(&self) -> u32 {
        self.blues
    }
}