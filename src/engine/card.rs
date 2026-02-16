
pub mod deck;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Card {
    #[default]
    Null,
    Red(CardNumber),
    Green(CardNumber),
    Yellow(CardNumber),
    Black(CardNumber),
    Rook,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardNumber {
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    One,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardSuit {
    Red,
    Green,
    Black,
    Yellow,
}

impl Card {

    pub fn points(&self) -> u32 {
        let inner = match self {
            Card::Null => return 0,
            Card::Red(card_number) => *card_number,
            Card::Green(card_number) => *card_number,
            Card::Yellow(card_number) => *card_number,
            Card::Black(card_number) => *card_number,
            Card::Rook => return 20,
        };
        inner.points()
    }

    pub fn suit(&self, trump: CardSuit) -> CardSuit {
        match self {
            Card::Null => panic!("Tried to get suit of Null card"),
            Card::Red(_) => CardSuit::Red,
            Card::Green(_) => CardSuit::Green,
            Card::Yellow(_) => CardSuit::Yellow,
            Card::Black(_) => CardSuit::Black,
            Card::Rook => trump,
        }
    }

    pub fn matches_suit(&self, trump: CardSuit) -> bool {
        self.suit(trump) == trump
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            Card::Null => panic!("goofy ahh computer"),
            Card::Red(card_number) => card_number.to_i32(),
            Card::Green(card_number) => card_number.to_i32(),
            Card::Yellow(card_number) => card_number.to_i32(),
            Card::Black(card_number) => card_number.to_i32(),
            Card::Rook => 1,
        }
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        *self == Self::Null
    }
}

impl CardNumber {
    pub fn points(&self) -> u32 {
        match self {
            Self::Six
            | Self::Seven
            | Self::Eight
            | Self::Nine
            | Self::Eleven
            | Self::Twelve
            | Self::Thirteen => 0,
            Self::Five => 5,
            Self::Ten | Self::Fourteen => 10,
            Self::One => 15,
        }
    }
    pub fn to_i32(&self) -> i32 {
        match self {
            CardNumber::Five => 5,
            CardNumber::Six => 6,
            CardNumber::Seven => 7,
            CardNumber::Eight => 8,
            CardNumber::Nine => 9,
            CardNumber::Ten => 10,
            CardNumber::Eleven => 11,
            CardNumber::Twelve => 12,
            CardNumber::Thirteen => 13,
            CardNumber::Fourteen => 14,
            CardNumber::One => 15,
        }
    }
}
