use std::fmt::Debug;

use rand::{Rng, rngs::ThreadRng, seq::SliceRandom};

use crate::engine::{
    card::{Card, CardSuit}, engine_player_state::EnginePlayerState, engine_state::EngineState,
};

pub mod card;
mod engine_player_state;
mod engine_state;
pub mod dummy_decision_engine;

// game stages
mod pregame;
mod bid;

#[derive(Debug)]
pub struct RookEngine {
    state: EngineState,
    rng: ThreadRng,
    players: [EnginePlayerState; 4],
}

pub trait RookPlayer: Debug {
    fn bid(&mut self, current_bid: u32, hand: [Card; 10]) -> Option<u32>;
    fn play_turn(&mut self, pot: [Card; 4], hand: [Card; 10]) -> usize;
    fn should_reshuffle(&mut self, hand: [Card; 10]) -> bool;
    fn chose_trump(&mut self, hand: [Card; 10], kitty: [Card;5]) -> CardSuit;
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
        let rng = rand::rng();
        let state = EngineState::Pregame;

        Self {
            state,
            rng,
            players,
        }
    }

    fn start_game(&mut self) {
        self.state = EngineState::Pregame;

        self.resume_and_complete_game();
    }

    fn resume_and_complete_game(&mut self) {
        while self.state != EngineState::Won {
            self.play_trick();
        }
    }

    pub fn play_trick(&mut self) {
        match self.state {
            EngineState::Pregame => {
                self.init_pregame();
                self.play_trick();
            }
            EngineState::Bid(kitty) => {
                let (trump, winner) = self.bid(kitty);
                println!("DID A BID");
                self.state = EngineState::Ingame(trump, winner);
            }
            EngineState::Ingame(trump_suit, turn) => {
                // Play tricks
            }
            EngineState::Won => {
                // someone won
            }
        }
    }

}

