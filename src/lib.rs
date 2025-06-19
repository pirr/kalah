use std::usize;

use rand::seq::{ IndexedRandom };
use rand;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct GameConfig {
    pub stone_nums_in_hole: usize,
    pub hole_nums: usize,
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Color {
    Blue,
    Red,
    Black,
    White,
    Yellow,
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Size {
    Small,
    Medium,
    Large,
}

pub struct Stone {
    pub color: Color,
    pub size: Size,
}

pub struct Hole {
    pub stones: Vec<Stone>,
}

pub struct Side {
    pub holes: Vec<Hole>,
}

pub struct Player {
    pub name: String,
    pub score: usize,
}

pub enum GameStatus {
    Run,
    Finished,
} 

pub struct GameField {
    pub side_one: Side,
    pub side_two: Side,
}

pub fn random_enum<T>() -> T
where
    T: IntoEnumIterator + Copy,
{
    let mut rng = rand::rng();
    *T::iter().collect::<Vec<_>>().choose(&mut rng).unwrap()
}

impl GameConfig {

    pub fn build(stone_nums_in_hole: usize, hole_nums: usize) -> GameConfig {
        GameConfig { 
            stone_nums_in_hole, 
            hole_nums,
        }
    }
}

impl GameField {

    pub fn build(config: &GameConfig) -> GameField {
        let mut side_one = Side { holes: Vec::new() };
        let mut side_two = Side { holes: Vec::new() };

        for side in [&mut side_one, &mut side_two] {
            for _ in 0..config.hole_nums {
                let mut hole = Hole { stones: Vec::new() };
                for _ in 0..config.stone_nums_in_hole {
                    let color: Color = random_enum();
                    let size: Size = random_enum();
                    let stone = Stone { color, size };
                    hole.stones.push(stone);
                }
                side.holes.push(hole);
            }
        }

        return GameField {
            side_one,
            side_two,
        };
    }

}

pub struct GameProcess {
    pub player_one: Player,
    pub player_two: Player,
    pub game_config: GameConfig,
    pub game_field: GameField,
    pub turn: i32,
    pub is_player_one_turn: bool,
    pub is_side_one: bool,
    pub total_turns: usize,
}

impl GameProcess {

    pub fn build(game_field: GameField, player_one_name: String, player_two_name: String, game_config: GameConfig) -> GameProcess {
        
        GameProcess { 
            player_one: Player {
                    name: player_one_name, 
                    score: 0 
                },
            player_two: Player { 
                    name: player_two_name, 
                    score: 0 
                }, 
            game_field: game_field, 
            turn: 0,
            game_config: game_config,
            is_player_one_turn: true,
            is_side_one: true,
            total_turns: 0,
        }
    }
    
    pub fn move_stones_from_hole(&mut self, hole_num: usize) -> Result<GameStatus, String> {

        if hole_num == 0 || hole_num > self.game_config.hole_nums {
            return Err(format!(
                "hole_num must be in range 1..={} (got {})",
                self.game_config.hole_nums, hole_num
            ));
        }

        let withdrawal_hole_indx = hole_num - 1;

        // Get the mutable reference to the current player's side
        let curr_side_mut = self.get_curren_side_mut();

        let stones = &mut curr_side_mut.holes[withdrawal_hole_indx]
            .stones
            .drain(..)
            .collect::<Vec<_>>();

        if stones.is_empty() {
            return Err("Selected hole is empty".into());
        }

        // Set up index and side references for distributing stones
        let mut hole_index = withdrawal_hole_indx + 1;
        let mut active_side = if self.is_player_one_turn {
            1
        } else {
            2
        };

        while !stones.is_empty() {

            let stone = stones.pop().unwrap();

            if hole_index == self.game_config.hole_nums {
                // Last hole, deposit into score
                if self.is_player_one_turn && active_side == 1  {
                    self.player_one.score += 1;
                }
                else if !self.is_player_one_turn && active_side == 2 {
                    self.player_two.score += 1;
                } else {
                    stones.push(stone);
                }

                // Switch sides and reset hole index
                hole_index = 0;
                active_side = if active_side == 1 { 2 } else { 1 };
            } else {
                let side = if active_side == 1 {
                    &mut self.game_field.side_one
                } else {
                    &mut self.game_field.side_two
                };

                side.holes[hole_index].stones.push(stone);
                hole_index += 1;
            }
        }

        if self.is_game_finish() { 
            self.finalize_score();
            return Ok(GameStatus::Finished);
        } 

        self.change_side(hole_index, active_side);
        
        // If last stone did not land in score, switch turn
        
        self.total_turns += 1;

        Ok(GameStatus::Run)
    }

    fn is_game_finish(&mut self) -> bool {
        let curr_side_mut = self.get_curren_side_mut();

        for hole in &curr_side_mut.holes {
            if !hole.stones.is_empty() {
                return false;
            }
        }
        true
    }

    fn get_curren_side_mut(&mut self) -> &mut Side {
        if self.is_player_one_turn {
            &mut self.game_field.side_one
        } else {
            &mut self.game_field.side_two
        }
    }

    fn finalize_score(&mut self) {
        let (side, player) = match self.is_player_one_turn {
            true => (&self.game_field.side_two, &mut self.player_two),
            _ => (&self.game_field.side_one, &mut self.player_one),
        };

        for hole in &side.holes {
            player.score += hole.stones.len();
        }

    }

    fn change_side(&mut self, hole_index: usize, active_side: u128) {
        // println!("Is player one turn {}, Active side {}", self.is_player_one_turn, active_side);

        if hole_index != 0 {
            self.is_player_one_turn = !self.is_player_one_turn;
        }

        else if !(self.is_player_one_turn && active_side == 2) && !(!self.is_player_one_turn && active_side == 1) {
            self.is_player_one_turn = !self.is_player_one_turn;
        }
    }
}
