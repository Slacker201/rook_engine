use crate::engine::{
    RookEngine,
    card::{Card, CardSuit},
    engine_player_state::EnginePlayerState,
    engine_state::{EngineState, Turn}, info_structs::PostTrickInformation,
};

impl RookEngine {
    pub fn play_trick(&mut self, trump: CardSuit, turn: Turn) {
        let nest = self.get_nest_cards();
        let mut players = self.get_players_arranged_to_turn(turn);
        let mut pot = [Card::Null; 4];
        let mut idx = 0;
        for player in players.iter_mut() {
            let card_played = player.play_turn(trump, pot);
            let card = player.get_and_remove(card_played);
            pot[idx] = card;
            idx += 1;
        }
        let winner_index = Self::get_index_of_winner(trump, pot);
        let winner = players.into_iter().nth(winner_index).unwrap();
        let winners_turn = winner.turn();

        winner.add_won_cards(pot);

        if winner.has_no_cards() {
            winner.add_nest(nest);
            self.state = EngineState::Won;
        } else {
            self.state = EngineState::Ingame(trump, Turn::from(winner_index));
        }
        let mut i = 0;
        let mut trick_info = PostTrickInformation {
            trick_winner: winners_turn,
            your_turn: Turn::from(0),
            trick_cards: pot,
        };

        for player in &mut self.players {
            trick_info.your_turn = Turn::from(i);
            i += 1;
            player.post_trick_info(&trick_info);
        }
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

    fn get_nest_cards(&mut self) -> [Card; 5] {
        let cards = self.nest.clone();
        self.nest = [Card::Null; 5];
        cards
    }
}
