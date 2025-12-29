use crate::engine::{RookEngine, card::{Card, CardSuit}, engine_player_state::EnginePlayerState, engine_state::Turn};



impl RookEngine {
    pub fn play_trick(&mut self, trump: CardSuit, turn: Turn) {

    }

    fn get_players_arranged_to_turn(&mut self, turn: Turn) -> [&mut EnginePlayerState; 4] {
        let mut players: Vec<&mut EnginePlayerState> = self.players.iter_mut().collect();
        let mut slice_1 = players.split_off(turn.to_idx());
        slice_1.extend(players);
        slice_1.try_into().unwrap()
    }
}