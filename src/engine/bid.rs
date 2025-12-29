use crate::engine::{RookEngine, card::{Card, CardSuit}, engine_player_state::EnginePlayerState, engine_state::Turn};



impl RookEngine {
    pub fn bid(&mut self, kitty: [Card; 5]) -> (CardSuit, Turn) {
        let players: Vec<&mut EnginePlayerState> = self.players.iter_mut().collect();
        let turns = Turn::new_turns();
        let mut bidders: Vec<(&mut EnginePlayerState, Turn)> = players.into_iter().zip(turns).collect();
        let mut idx = 0;

        let mut current_bid = 0;
        while bidders.len() > 1 {
            let bid = bidders[idx].0.bid(current_bid);
            match bid {
                Some(v) => {
                    if v > current_bid && v < 180 {
                        current_bid = v;
                        idx += 1;
                    } else {
                        bidders.remove(idx);
                    }
                },
                None => {
                    bidders.remove(idx);
                }
            }
            if idx >= bidders.len() {
                idx = 0
            }
        }
        (bidders[0].0.chose_trump(kitty), bidders[0].1)
    }
}