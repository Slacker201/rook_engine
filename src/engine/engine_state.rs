use crate::engine::card::{Card, CardSuit};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EngineState {
    #[default]
    Pregame,
    Bid([Card; 5]),
    Ingame(CardSuit, Turn),
    Won,
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Turn {
    One,
    Two,
    Three,
    Four,
}

impl Turn {
    pub fn new_turns() -> [Turn; 4] {
        [
            Turn::One,
            Turn::Two,
            Turn::Three,
            Turn::Four,
        ]
    }
    pub fn to_idx(&self) -> usize {
        match self {
            Turn::One => 0,
            Turn::Two => 1,
            Turn::Three => 2,
            Turn::Four => 3,
        }
    }
}


impl From<usize> for Turn {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::One,
            1 => Self::Two,
            2 => Self::Three,
            3 => Self::Four,
            _ => panic!("please use 0-3 as input")
        }
    }
}