use crate::{dummy_decision_engine::DummyEngine, engine::RookEngine};




pub mod engine;
pub mod dummy_decision_engine;

fn main() {
    let mut engine = RookEngine::new([
        Box::new(DummyEngine::new()),
        Box::new(DummyEngine::new()),
        Box::new(DummyEngine::new()),
        Box::new(DummyEngine::new()),
        ]);
    engine.advance_game();
    println!("{:?}", engine);
    engine.advance_game();
    println!("{:?}", engine);
}
