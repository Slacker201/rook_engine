use rand::{rng, seq::SliceRandom};

use crate::engine::{RookEngine, card::Card, dummy_decision_engine::DummyEngine};

pub mod engine;

fn main() {
    let mut engine = RookEngine::new([
        Box::new(DummyEngine::new()),
        Box::new(DummyEngine::new()),
        Box::new(DummyEngine::new()),
        Box::new(DummyEngine::new()),
        ]);
    engine.play_trick();
    println!("{:?}", engine)
}
