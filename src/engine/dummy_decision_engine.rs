use crate::engine::{RookPlayer, card::CardSuit};


#[derive(Debug)]
pub struct DummyEngine {

}

impl RookPlayer for DummyEngine {
    fn bid(&mut self, current_bid: u32, hand: [super::card::Card; 10]) -> Option<u32> {
        if current_bid >= 120 {
            None
        } else {
            Some(current_bid + 10)
        }
    }

    fn play_turn(&mut self, trump: CardSuit, pot: [super::card::Card; 4], hand: [super::card::Card; 10]) -> usize {
        0
    }

    fn should_reshuffle(&mut self, hand: [super::card::Card; 10]) -> bool {
        false
    }
    fn chose_trump(&mut self, hand: [super::card::Card; 10]) -> CardSuit {
        CardSuit::Black
    }
    
    fn chose_hand(&mut self, hand: [super::card::Card; 15]) -> [super::card::Card; 10] {
        hand[0..10].try_into().unwrap()
    }

}

impl DummyEngine {
    pub fn new() -> Self {
        Self {  }
    }
}