use crate::engine::{RookPlayer, card::{Card, CardSuit}};

#[derive(Debug)]
pub struct EnginePlayerState {
    hand: [Card; 10],
    won_cards: Vec<Card>,
    decision_maker: Box<dyn RookPlayer>,
}

impl EnginePlayerState {
    pub fn new(hand: [Card; 10], decision_maker: Box<dyn RookPlayer>) -> EnginePlayerState {
        EnginePlayerState {
            hand,
            won_cards: Vec::new(),
            decision_maker,
        }
    }

    pub fn set_hand(&mut self, new_hand: [Card; 10]) {
        self.hand = new_hand;
    }

    pub fn player_has_sufficient_points(&mut self) -> bool {
        let mut points_total = 0;
        for card in self.hand {
            points_total += card.points();
        }
        println!("points_total: {}", points_total);
        if points_total > 15 {
            true
        } else if points_total == 15 {
            self.decision_maker.should_reshuffle(self.hand)
        } else {
            false
        }
    }
    pub fn bid(&mut self, current_bid: u32) -> Option<u32> {
        self.decision_maker.bid(current_bid, self.hand)
    }
    pub fn chose_trump(&mut self, kitty: [Card; 5]) -> CardSuit {
        self.decision_maker.chose_trump(self.hand, kitty)
    }
}
