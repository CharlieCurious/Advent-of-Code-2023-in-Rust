use super::bag::Bag;

#[derive(Debug, PartialEq)]
pub struct CubePermutation {
    red: u32,
    green: u32,
    blue: u32
}

impl CubePermutation {
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        CubePermutation { red, green, blue }
    }

    pub fn is_possible(&self, bag: &Bag) -> bool {
        return self.red <= bag.get_number_of_reds() &&
            self.green <= bag.get_number_of_greens() &&
            self.blue <= bag.get_number_of_blues()
    }
}