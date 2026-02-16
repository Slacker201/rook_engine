use crate::engine::{
    RookPlayer,
    card::{Card, CardSuit},
    engine_state::Turn,
    info_structs::{PostBidInformation, PostTrickInformation},
};

#[derive(Debug)]
pub struct EnginePlayerState {
    hand: [Card; 10],
    won_cards: Vec<Card>,
    decision_maker: Box<dyn RookPlayer>,
    turn: Turn,
}

impl EnginePlayerState {
    pub fn new(
        hand: [Card; 10],
        decision_maker: Box<dyn RookPlayer>,
        turn: Turn,
    ) -> EnginePlayerState {
        EnginePlayerState {
            hand,
            won_cards: Vec::new(),
            decision_maker,
            turn,
        }
    }

    pub fn set_hand(&mut self, new_hand: [Card; 10]) {
        self.hand = new_hand;
    }

    pub fn bid(&mut self, current_bid: u32) -> Option<u32> {
        self.decision_maker.bid(current_bid, self.hand)
    }

    pub fn chose_trump(&mut self) -> CardSuit {
        self.decision_maker.chose_trump(self.hand)
    }

    fn select_card_of_suit(&self, suit: CardSuit, trump: CardSuit) -> Option<usize> {
        for idx in 0..11 {
            if self.hand[idx] != Card::Null && self.hand[idx].suit(trump) == suit {
                return Some(idx)
            }
        }
        None
    }

    pub fn play_turn(&mut self, trump: CardSuit, pot: [Card; 4]) -> usize {
        let mut selected_card = self.decision_maker.play_turn(trump, pot, self.hand);
        if self.hand[selected_card].is_null() {
            for idx in 0..11 {
                if !self.hand[idx].is_null() {
                    selected_card = idx;
                    break;
                }
            }
        }
        if pot[0].is_null() || pot[0].suit(trump) == self.hand[selected_card].suit(trump) {
            return selected_card;
        }
        self.select_card_of_suit(pot[0].suit(trump), trump).unwrap_or(selected_card)
    }

    pub fn get_and_remove(&mut self, card_idx: usize) -> Card {
        let card = self.hand[card_idx];
        self.hand[card_idx] = Card::Null;
        card
    }

    pub fn add_won_cards(&mut self, pot: [Card; 4]) {
        self.won_cards.extend_from_slice(&pot);
    }

    pub fn has_no_cards(&self) -> bool {
        self.hand.iter().all(|card| card == &Card::Null)
    }

    pub fn chose_hand(&mut self, kitty: [Card; 5]) -> ([Card; 10], [Card; 5]) {
        let mut expanded_hand = self.hand.to_vec();
        expanded_hand.extend_from_slice(&kitty);
        self.decision_maker
            .chose_hand(expanded_hand.try_into().unwrap())
    }

    pub fn add_nest(&mut self, nest: [Card; 5]) {
        self.won_cards.extend_from_slice(&nest);
    }

    pub fn score(&self) -> u32 {
        self.won_cards.iter().map(|card| card.points()).sum()
    }

    pub fn post_bid_info(&mut self, bid_info: &PostBidInformation) {
        self.decision_maker.post_bid_information(bid_info);
    }

    pub fn post_trick_info(&mut self, trick_info: &PostTrickInformation) {
        self.decision_maker.post_trick_information(trick_info);
    }

    pub fn turn(&self) -> Turn {
        self.turn
    }

    pub fn should_reshuffle(&mut self) -> bool {
        self.decision_maker.should_reshuffle(self.hand)
    }
}
