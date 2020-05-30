use nerdee_engine::engine_core::*;
use nerdee_engine::terrain::Terrain;
use nerdee_engine::thread_pool::ThreadPool;
use std::sync::Arc;

use std::thread;

fn main() {
    let mut engine = Engine::new(8);
    engine.add(Box::new(Terrain::new()));
    engine.run_forever();
}