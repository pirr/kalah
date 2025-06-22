pub struct GameConfig {
    pub stone_nums_in_hole: usize,
    pub hole_nums: usize,
}


impl GameConfig {

    pub fn build(stone_nums_in_hole: usize, hole_nums: usize) -> GameConfig {
        GameConfig { 
            stone_nums_in_hole, 
            hole_nums,
        }
    }
}
