use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::engine::card::{Card, CardNumber};



pub struct Deck {
    cards: [Card; 45]
}

pub trait CardScoring {
    fn points(&self) -> u32;
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
        let players = loop {
            // shuffle the deck
            self.shuffle(&mut rng);

            // generate the players cards
            let players: [[Card; 10]; 4] = [
                self.cards[0..10].try_into().unwrap(),
                self.cards[10..20].try_into().unwrap(),
                self.cards[20..30].try_into().unwrap(),
                self.cards[30..40].try_into().unwrap(),
            ];

            // check if we should reshuffle
            let should_reshuffle = players.iter().all(|vector| {
                let score = vector.points();
                if score >= 15 {
                    return false
                } else if score < 15 {
                    return true
                } else {
                    false
                }
            });
            if !should_reshuffle {
                break players;
            }
        };
        let nest = self.cards[40..45]
            .try_into()
            .expect("Somehow the atoms of the universe conspired against you!");
        (players, nest)
    }
}

impl CardScoring for [Card] {
    fn points(&self) -> u32 {
        let mut score = 0;
        for card in self {
            score += card.points()
        }
        score
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