use rand::seq::SliceRandom;

use crate::engine::{RookEngine, card::Card, engine_state::EngineState};

impl RookEngine {
    pub fn init_pregame(&mut self) {
        let mut players_have_sufficient_points = false;
        while !players_have_sufficient_points {
            println!("Shuffling Cards");
            players_have_sufficient_points = true;
            let mut deck = Card::new_deck();
            deck.shuffle(&mut self.rng);
            // dole out cards

            // pick 10 cards for each player
            let mut i = 0;

            for player in &mut self.players {
                player.set_hand(deck[i..i + 10].try_into().unwrap());
                if !player.player_has_sufficient_points() {
                    players_have_sufficient_points = false;
                    println!("A player has insufficient points :(");
                    break;
                }
                i += 10;
            }
            self.state = EngineState::Bid(deck[40..45].try_into().unwrap())
        }
    }
}
