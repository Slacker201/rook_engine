use crate::engine::{card::CardSuit, engine_state::Turn};



pub struct PostBidInformation {
    bid_winner: Turn,
    your_turn: Turn,
    won_bid: u32,
    trump_suit: CardSuit,
}