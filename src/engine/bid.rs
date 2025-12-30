use crate::engine::{
    RookEngine,
    card::{Card, CardSuit},
    engine_player_state::EnginePlayerState,
    engine_state::Turn,
    info_structs::PostBidInformation,
};

impl RookEngine {
    pub fn bid(&mut self, kitty: [Card; 5]) -> (CardSuit, Turn) {
        let players: Vec<&mut EnginePlayerState> = self.players.iter_mut().collect();
        let turns = Turn::new_turns();
        let mut bidders: Vec<(&mut EnginePlayerState, Turn)> =
            players.into_iter().zip(turns).collect();
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
                }
                None => {
                    bidders.remove(idx);
                }
            }
            if idx >= bidders.len() {
                idx = 0
            }
        }

        let (bid_winner, turn) = bidders.iter_mut().nth(0).unwrap();
        let turn = *turn;
        let (new_hand, nest) = bid_winner.chose_hand(kitty);
        bid_winner.set_hand(new_hand);
        self.nest = nest;
        self.bid = current_bid;
        let trump = bid_winner.chose_trump();
        let mut i = 0;
        let mut bid_info = PostBidInformation {
            bid_winner: turn,
            your_turn: Turn::from(0),
            won_bid: current_bid,
            trump_suit: trump,
        };
        for player in &mut self.players {
            bid_info.your_turn = Turn::from(i);
            i += 1;
            player.post_bid_info(&bid_info);
        }
        (trump, turn)
    }
}
