use crate::engine::card::Card;



pub struct Deck {
    cards: [Card; 45]
}

impl Deck {
    pub fn new() {
        let cards = Card::new_deck();
    }
}