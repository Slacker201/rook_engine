use std::fmt::Debug;

use rand::rngs::ThreadRng;

use crate::engine::{
    card::{Card, CardSuit},
    engine_player_state::EnginePlayerState,
    engine_state::{EngineState, Turn},
};

pub mod card;
mod engine_player_state;
mod engine_state;

// game stages
mod bid;
mod ingame;
mod pregame;
mod game_won;

#[derive(Debug)]
pub struct RookEngine {
    state: EngineState,
    rng: ThreadRng,
    players: [EnginePlayerState; 4],
    nest: [Card; 5],
    bid: u32,
    bid_winner: Turn,
}

pub trait RookPlayer: Debug {
    fn bid(&mut self, current_bid: u32, hand: [Card; 10]) -> Option<u32>;
    fn play_turn(&mut self, trump: CardSuit, pot: [Card; 4], hand: [Card; 10]) -> usize;
    fn should_reshuffle(&mut self, hand: [Card; 10]) -> bool;
    fn chose_trump(&mut self, hand: [Card; 10]) -> CardSuit;
    fn chose_hand(&mut self, hand: [Card; 15]) -> ([Card; 10], [Card; 5]);
}

impl RookEngine {
    pub fn new(decision_makers: [Box<dyn RookPlayer>; 4]) -> Self {
        const EMPTY_HAND: [Card; 10] = [Card::Null; 10];
        let [p1, p2, p3, p4] = decision_makers;
        let players = [
            EnginePlayerState::new(EMPTY_HAND, p1),
            EnginePlayerState::new(EMPTY_HAND, p2),
            EnginePlayerState::new(EMPTY_HAND, p3),
            EnginePlayerState::new(EMPTY_HAND, p4),
        ];

        let empty_nest = [Card::Null; 5];
        let rng = rand::rng();
        let state = EngineState::Pregame;

        Self {
            state,
            rng,
            players,
            nest: empty_nest,
            bid: 0,
            bid_winner: Turn::One,
        }
    }

    pub fn play_game(&mut self) {
        self.state = EngineState::Pregame;

        self.resume_and_complete_game();
    }

    pub fn resume_and_complete_game(&mut self) {
        while self.state != EngineState::Won {
            self.advance_game();
        }
        self.advance_game();
    }

    pub fn advance_game(&mut self) {
        match self.state {
            EngineState::Pregame => {
                self.init_pregame();
                self.advance_game();
            }
            EngineState::Bid(kitty) => {
                let (trump, winner) = self.bid(kitty);
                self.bid_winner = winner;
                println!("DID A BID");
                self.state = EngineState::Ingame(trump, winner);
            }
            EngineState::Ingame(trump_suit, turn) => {
                // Play tricks
                self.play_trick(trump_suit, turn);
            }
            EngineState::Won => {
                // someone won
                // iterate through each players won cards and see who wins
                self.game_won();
            }
        }
    }
}
