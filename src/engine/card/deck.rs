use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::engine::card::{Card, CardNumber};



pub struct Deck {
    cards: [Card; 45]
}

impl Deck {
    pub fn new() -> Self {
        let cards = Card::new_deck();
        Self { cards }
    }
    fn shuffle(&mut self, rng: &mut ThreadRng) {
        self.cards.shuffle(rng);
    }
    pub fn get_player_cards(mut self) -> ([[Card; 10]; 4], [Card; 5]) {
        let mut rng = rand::rng();
        self.shuffle(&mut rng);
        let players = [
            self.cards[0..10].try_into().unwrap(),
            self.cards[10..20].try_into().unwrap(),
            self.cards[20..30].try_into().unwrap(),
            self.cards[30..40].try_into().unwrap(),
        ];
        let nest = self.cards[40..45]
            .try_into()
            .expect("Somehow the atoms of the universe conspired against you!");
        (players, nest)
    }
}

impl Card {
    fn new_deck() -> [Card; 45] {
        const NUMBERS: [CardNumber; 11] = [
            CardNumber::Five,
            CardNumber::Six,
            CardNumber::Seven,
            CardNumber::Eight,
            CardNumber::Nine,
            CardNumber::Ten,
            CardNumber::Eleven,
            CardNumber::Twelve,
            CardNumber::Thirteen,
            CardNumber::Fourteen,
            CardNumber::One,
        ];
        let mut cards = Vec::new();
        for i in 0..4 {
            for item in NUMBERS {
                cards.push(match i {
                    0 => Card::Red(item),
                    1 => Card::Green(item),
                    2 => Card::Yellow(item),
                    3 => Card::Black(item),
                    _ => unreachable!(),
                });
            }
        }
        cards.push(Card::Rook);
        cards.try_into().unwrap()
    }
}