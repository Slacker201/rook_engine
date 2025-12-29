use crate::engine::{RookEngine, card::{Card, CardSuit}, engine_player_state::EnginePlayerState, engine_state::Turn};



impl RookEngine {
    pub fn play_trick(&mut self, trump: CardSuit, turn: Turn) {
        let players = self.get_players_arranged_to_turn(turn);
        let mut pot = [Card::Null; 4];
        let mut idx = 0;
        for player in players {
            let card_played = player.play_turn(trump, pot);
            let card = player.get_and_remove(card_played);
            pot[idx] = card;
            idx += 1;
        }

        let winner = players[Self::get_index_of_winner(pot)];

    }

    fn get_players_arranged_to_turn(&mut self, turn: Turn) -> [&mut EnginePlayerState; 4] {
        let mut players: Vec<&mut EnginePlayerState> = self.players.iter_mut().collect();
        let mut slice_1 = players.split_off(turn.to_idx());
        slice_1.extend(players);
        slice_1.try_into().unwrap()
    }

    fn get_index_of_winner(trump: CardSuit, pot: [Card; 4]) -> usize {
        let first_card_suit = pot[0].suit();
    }
}