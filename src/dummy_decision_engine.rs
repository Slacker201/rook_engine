use crate::engine::{RookPlayer, card::{Card, CardSuit}};


#[derive(Debug)]
pub struct DummyEngine {

}

impl RookPlayer for DummyEngine {
    fn bid(&mut self, current_bid: u32, hand: [Card; 10]) -> Option<u32> {
        if current_bid >= 120 {
            None
        } else {
            Some(current_bid + 10)
        }
    }

    fn play_turn(&mut self, trump: CardSuit, pot: [Card; 4], hand: [Card; 10]) -> usize {
        0
    }

    fn should_reshuffle(&mut self, hand: [Card; 10]) -> bool {
        false
    }
    fn chose_trump(&mut self, hand: [Card; 10]) -> CardSuit {
        CardSuit::Black
    }
    
    fn chose_hand(&mut self, hand: [Card; 15]) -> ([Card; 10], [Card; 5]) {
        (hand[0..10].try_into().unwrap(), hand[10..15].try_into().unwrap())
    }

}

impl DummyEngine {
    pub fn new() -> Self {
        Self {  }
    }
}