use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use crate::thread_pool::ThreadPool;
use crate::engine_core::*;

/// Holds a chunk of the terrain
struct Chunk {
    id: usize,
}

impl Chunk {
    /// Returns a chunk with the given id
    fn new(id: usize) -> Chunk {
        Chunk { id }
    }

    /// Updates the chunk
    fn update(&mut self) {
        thread::sleep_ms(2);
    }

    /// Renders the chunk to screen
    fn draw(&self) {
        thread::sleep_ms(2);
    }
}

/// A continuously streaming terrain generator
pub struct Terrain {
    chunks: Vec<Arc<Mutex<Chunk>>>,
}

impl Terrain {
    /// Returns a new Terrain object that can be added to the engine
    pub fn new() -> Terrain {
        let chunks = (0..10).map(|i| Arc::new(Mutex::new(Chunk::new(i)))).collect();

        Terrain { chunks }
    }
}


impl EngineObject for Terrain {

    /// Not yet needed
    fn start_up(&mut self, core: &Core) {

    }

    /// Saves each chunk to a save folder
    fn save(&self, core: &Core) {

    }

    /// Loads each chunk from a save folder
    fn load(&mut self, core: &Core) {

    }
    
    /// Updates each Chunk using a ThreadPool
    fn update(&mut self, core: &Core) {
        for chunk in &self.chunks {
            let chunk = chunk.clone();
            core.execute(move || {
                let mut chunk = chunk.lock().unwrap();
                chunk.update();
            });
        }
    }

    /// Renders each Chunk using a ThreadPool
    fn draw(&self, core: &Core) {
        for chunk in &self.chunks {
            let chunk = chunk.clone();
            core.execute(move || {
                let chunk = chunk.lock().unwrap();
                chunk.draw();
            });
        }
    }
}