use crate::engine::{RookEngine, card::{deck::{CardScoring, Deck}}, engine_state::EngineState};

impl RookEngine {
    pub fn init_pregame(&mut self) {
        let mut should_reshuffle = true;
        while should_reshuffle {
            should_reshuffle = false;
            println!("Shuffling Cards");
            let deck = Deck::new();
            let cards = deck.get_player_cards();

            // pick 10 cards for each player
            let mut i = 0;
            for hand in cards.0 {
                self.players[i].set_hand(hand);
                if hand.points() == 15 {
                    should_reshuffle = self.players[i].should_reshuffle() || should_reshuffle;
                }
                i += 1;
            }
            self.state = EngineState::Bid(cards.1)
        }
    }
}
