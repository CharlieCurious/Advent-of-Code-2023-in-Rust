use super::cube_permutation::CubePermutation;

#[derive(Debug, PartialEq)]
pub struct Game {
    id: u32,
    permutations: Vec<CubePermutation>
}

impl Game {
    pub fn new(id: u32, permutations: Vec<CubePermutation>) -> Self {
        Game { id, permutations }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_permutations(&self) -> &Vec<CubePermutation> {
        &self.permutations
    }
}