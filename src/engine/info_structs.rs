use crate::engine::{
    card::{Card, CardSuit},
    engine_state::Turn,
};

#[derive(Debug, Clone, Copy)]
pub struct PostBidInformation {
    pub bid_winner: Turn,
    pub your_turn: Turn,
    pub won_bid: u32,
    pub trump_suit: CardSuit,
}

#[derive(Debug, Clone, Copy)]
pub struct PostTrickInformation {
    pub trick_winner: Turn,
    pub your_turn: Turn,
    pub trick_cards: [Card; 4],
}
