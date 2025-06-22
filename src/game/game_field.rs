use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rand::seq::{ IndexedRandom };
use rand;

use super::game_config::GameConfig;

pub struct GameField {
    pub side_one: Side,
    pub side_two: Side,
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

pub fn random_enum<T>() -> T
where
    T: IntoEnumIterator + Copy,
{
    let mut rng = rand::rng();
    *T::iter().collect::<Vec<_>>().choose(&mut rng).unwrap()
}
