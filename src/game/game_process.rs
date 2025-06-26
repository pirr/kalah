use super::game_config::GameConfig;
use super::game_field::{ GameField, Side };
use super::player::{ Player};
use super::game_status::GameStatus;

pub struct GameProcess {
    pub player_one: Player,
    pub player_two: Player,
    pub game_config: GameConfig,
    pub game_field: GameField,
    pub is_player_one_turn: bool,
    pub total_turns: usize,
    #[cfg(feature = "test_hooks")]
    pub swap_side_called: bool,
    #[cfg(feature = "test_hooks")]
    pub swap_players_called: bool,
}


pub struct LastTurnHole {
    pub side: u8,
    pub hole_index: usize,
    pub player_num: u8,
}

impl GameProcess {

    pub fn build(game_field: GameField, player_one_name: String, player_two_name: String, game_config: GameConfig) -> GameProcess {
        
        let game_process = GameProcess { 
            player_one: Player {
                    name: player_one_name, 
                    score: 0 
                },
            player_two: Player { 
                    name: player_two_name, 
                    score: 0 
                }, 
            game_field: game_field, 
            game_config: game_config,
            is_player_one_turn: true,
            total_turns: 0,
            #[cfg(feature = "test_hooks")]
            swap_side_called: false,
            #[cfg(feature = "test_hooks")]
            swap_players_called: false,
        };

        game_process
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

        let mut last_turn_hole = LastTurnHole {side: 1, hole_index: 0, player_num: 1};

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
                last_turn_hole.hole_index = hole_index;
                last_turn_hole.side = active_side;
                active_side = if active_side == 1 { 2 } else { 1 };
            } else {
                let side = if active_side == 1 {
                    &mut self.game_field.side_one
                } else {
                    &mut self.game_field.side_two
                };

                side.holes[hole_index].stones.push(stone);
                last_turn_hole.hole_index = hole_index;
                last_turn_hole.side = active_side;
                hole_index += 1;
            }
        }

        let single_stone_score = self.last_stone_single_score(&last_turn_hole);
        let player = match self.is_player_one_turn {
            true => &mut self.player_one,
            _ => &mut self.player_two,
        };
        player.score += single_stone_score;

        self.total_turns += 1;

        if self.is_game_finish() { 
            self.finalize_score();
            return Ok(GameStatus::Finished);
        }

        self.change_side(hole_index, active_side);

        if self.game_config.is_check_pipe_rule && self.check_pie_rule(&last_turn_hole) {
            self.swap_side();
            self.swap_players_score();
        };

        Ok(GameStatus::Run)
    }

    fn check_pie_rule(&self, last_turn_hole: &LastTurnHole) -> bool {
        if last_turn_hole.hole_index == 0 && self.total_turns == 1 && self.is_player_one_turn {
            return true;
        }
        false
    }

    fn swap_side(&mut self) {
        std::mem::swap(&mut self.game_field.side_one, &mut self.game_field.side_two);
        #[cfg(feature = "test_hooks")]
        {
            self.swap_side_called = true;
        }
    }

    fn swap_players_score(&mut self) {
        std::mem::swap(&mut self.player_one.score, &mut self.player_two.score);
        #[cfg(feature = "test_hooks")]
        {
            self.swap_players_called = true;
        }
    }

    fn is_game_finish(&mut self) -> bool {
        let is_empty_side = [&self.game_field.side_one.holes, &self.game_field.side_two.holes]
            .iter()
            .map(|holes| {
                holes.iter().filter(|hole| hole.stones.len() > 0).next().is_none()
            })
            .any(|is_empty| is_empty);

        return is_empty_side;
    }

    fn get_curren_side_mut(&mut self) -> &mut Side {
        if self.is_player_one_turn {
            &mut self.game_field.side_one
        } else {
            &mut self.game_field.side_two
        }
    }

    fn get_curren_side(&self) -> &Side {
        if self.is_player_one_turn {
            &self.game_field.side_one
        } else {
            &self.game_field.side_two
        }
    }

    fn finalize_score(&mut self) {
        for (side, player) in [(&self.game_field.side_one, &mut self.player_one), (&self.game_field.side_two, &mut self.player_two)] {
            for hole in &side.holes {
                player.score += hole.stones.len();
            }
        }
    }

    fn change_side(&mut self, hole_index: usize, active_side: u8) {
        // println!("Is player one turn {}, Active side {}", self.is_player_one_turn, active_side);

        if hole_index != 0 {
            self.is_player_one_turn = !self.is_player_one_turn;
        }

        else if !(self.is_player_one_turn && active_side == 2) && !(!self.is_player_one_turn && active_side == 1) {
            self.is_player_one_turn = !self.is_player_one_turn;
        }
    }

    fn last_stone_single_score(&mut self, last_turn_hole: &LastTurnHole) -> usize {

        if self.is_player_one_turn && last_turn_hole.side == 1 && self.get_curren_side().holes[last_turn_hole.hole_index].stones.len() == 1 {
            return self.game_field.side_two.holes[self.game_config.hole_nums - last_turn_hole.hole_index - 1].stones.drain(..).collect::<Vec<_>>().len();
        }

        else if !self.is_player_one_turn && last_turn_hole.side == 2 && self.get_curren_side().holes[last_turn_hole.hole_index].stones.len() == 1 {
           return self.game_field.side_one.holes[self.game_config.hole_nums - last_turn_hole.hole_index - 1].stones.drain(..).collect::<Vec<_>>().len();
        }

        0
    }
}
