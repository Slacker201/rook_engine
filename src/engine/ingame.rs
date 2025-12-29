use crate::engine::{RookEngine, card::{Card, CardSuit}, engine_player_state::EnginePlayerState, engine_state::Turn};



impl RookEngine {
    pub fn play_trick(&mut self, trump: CardSuit, turn: Turn) {
        let mut players = self.get_players_arranged_to_turn(turn);
        let mut pot = [Card::Null; 4];
        let mut idx = 0;
        for player in players.iter_mut() {
            let card_played = player.play_turn(trump, pot);
            let card = player.get_and_remove(card_played);
            pot[idx] = card;
            idx += 1;
        }

        let mut winner = players.into_iter().nth(Self::get_index_of_winner(trump, pot)).unwrap();

        winner.add_won_cards(pot);
    }

    fn get_players_arranged_to_turn(&mut self, turn: Turn) -> [&mut EnginePlayerState; 4] {
        let mut players: Vec<&mut EnginePlayerState> = self.players.iter_mut().collect();
        let mut slice_1 = players.split_off(turn.to_idx());
        slice_1.extend(players);
        slice_1.try_into().unwrap()
    }

    fn get_index_of_winner(trump: CardSuit, pot: [Card; 4]) -> usize {
        let first_card_suit = pot[0].suit(trump);
        let mut highest_first_suit = 0;
        let mut highest_trump = 0;
        let mut idx = 0;
        let mut trump_used = false;
        let mut highest_idx = 0;
        for card in pot {
            if card.matches_suit(trump) {
                trump_used = true;
                let card_value = card.to_i32();
                if card_value > highest_trump {
                    highest_trump = card_value;
                    highest_idx = idx;
                }
            } else {
                if card.matches_suit(first_card_suit) {
                    let card_value = card.to_i32();
                    if card_value > highest_first_suit && !trump_used {
                        highest_first_suit = card_value;
                        highest_idx = idx;
                    }
                }
            }
            idx += 1;
        }
        
        highest_idx
    }
}