use rand::seq::{ IndexedRandom };
use rand;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct GameConfig {
    pub stone_nums_in_hole: u128,
    pub hole_nums: u128,
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
    pub score: u32,
}

// impl Player {
//     pub fn choose_hole(hole: )
// }

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

    pub fn build(args: &[&str; 3]) -> Result<GameConfig, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }

        let stone_nums_in_hole: u128 = args[1]
            .parse()
            .map_err(|_| "Stone number in hole should be a number")?;

        let hole_nums: u128 = args[2]
            .parse()
            .map_err(|_| "Hole number should be a number")?;

        Ok(GameConfig { 
            stone_nums_in_hole, 
            hole_nums,
        })
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
    game_field: GameField,
    pub turn: i32,
    pub is_player_one_turn: bool,
}

// impl GameProcess {

//     pub fn build(game_field: GameField, player_one_name: String, player_two_name: String) -> GameProcess {
        
//         GameProcess { 
//             player_one: Player {
//                     name: player_one_name, 
//                     score: 0 
//                 },
//             player_two: Player { 
//                     name: player_two_name, 
//                     score: 0 
//                 }, 
//             game_field: game_field, 
//             turn: 0,
//             is_player_one_turn: true 
//         }
//     }
    
//     pub fn move_stone(&mut self) {

//         if self.is_player_one_turn {
//             pl
//         }

//     }
// }