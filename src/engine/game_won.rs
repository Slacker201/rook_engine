

use crate::engine::{RookEngine, engine_state::Turn};

#[derive(Debug)]
enum Team {
    TeamOne,
    TeamTwo,
}


impl RookEngine {
    pub fn game_won(&mut self) {
        // find winner
        let bidder_score = {
            match self.get_bid_team() {
                Team::TeamOne => {
                    self.players[0].score() + self.players[2].score()
                },
                Team::TeamTwo => {
                    self.players[1].score() + self.players[3].score()
                },
            }
        };
        if bidder_score >= self.bid {
            println!("Winner is {:?}", self.get_bid_team())
        } else {

            println!("Winner is {:?}", self.get_bid_team().swap())
        }
    }

    fn get_bid_team(&self) -> Team {
        match self.bid_winner {
            Turn::One | Turn::Three => Team::TeamOne,
            Turn::Two | Turn::Four => Team::TeamTwo,
        }
    }
}

impl Team {
    fn swap(self) -> Self {
        match self {
            Team::TeamOne => Team::TeamTwo,
            Team::TeamTwo => Team::TeamOne,
        }
    }
}