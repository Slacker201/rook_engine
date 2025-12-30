use crate::engine::{
    card::{Card, CardSuit},
    engine_state::Turn,
};

pub struct PostBidInformation {
    pub bid_winner: Turn,
    pub your_turn: Turn,
    pub won_bid: u32,
    pub trump_suit: CardSuit,
}

pub struct PostTrickInformation {
    pub trick_winner: Turn,
    pub your_turn: Turn,
    pub trick_cards: [Card; 4],
}
