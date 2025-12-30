use crate::engine::{card::{CardSuit}, engine_state::Turn};



pub struct PostBidInformation {
    pub bid_winner: Turn,
    pub your_turn: Turn,
    pub won_bid: u32,
    pub trump_suit: CardSuit,
}