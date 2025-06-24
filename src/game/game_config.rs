pub struct GameConfig {
    pub stone_nums_in_hole: usize,
    pub hole_nums: usize,
    pub is_check_pipe_rule: bool,
}


impl GameConfig {

    pub fn build(stone_nums_in_hole: usize, hole_nums: usize, is_check_pipe_rule: bool) -> Result<GameConfig, &'static str> {
        if stone_nums_in_hole > 9 {
            return Err("Max stone numbers in hole are 9");
        }

        if hole_nums > 9 {
            return Err("Max hole numbers are 9");
        }

        Ok(GameConfig { 
            stone_nums_in_hole, 
            hole_nums,
            is_check_pipe_rule,
        })
    }
}
