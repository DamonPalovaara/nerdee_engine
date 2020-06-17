use std::sync::{Arc, Mutex};
use std::thread;
use crate::engine_core::*;
use noise::{NoiseFn, Fbm};

const CHUNK_BLOCKS: usize = 512;
const CHUNK_RADIUS: isize = (CHUNK_BLOCKS / 2) as isize;
const BLOCK_SIZE:   f64   = 0.5;
const CHUNK_SIZE:   f64   = CHUNK_BLOCKS as f64 * BLOCK_SIZE;

// Temporary until I decide on a math library
type Point = (f32, f32, f32);

struct Coordinate {
    x: isize,
    y: isize
}

impl From<Point> for Coordinate {
    fn from(point: Point) -> Self {
        let x = (point.0 / CHUNK_SIZE as f32).round() as isize;
        let y = (point.1 / CHUNK_SIZE as f32).round() as isize;
        Coordinate { x, y }
    }
}

/// Holds a chunk of the terrain
struct Chunk {
    coordinate: Coordinate,
    mesh:       Vec<Point>,
}

impl Chunk {
    /// Returns a chunk with the given id
    fn new(x: isize, y: isize) -> Chunk {
        let coordinate = Coordinate { x, y };
        let mesh = Vec::new();
        Chunk { coordinate, mesh }
    }

    fn generate(&mut self, noise: Arc<Fbm>) {
        println!("Generating chunk: ({}, {})", self.coordinate.x, self.coordinate.y);

        for z in (-1 * CHUNK_RADIUS)..CHUNK_RADIUS {
            for x in (-1 * CHUNK_RADIUS)..CHUNK_RADIUS {
                let x_loc = x as f64 * BLOCK_SIZE;
                let z_loc = z as f64 * BLOCK_SIZE;
                let y_loc = noise.get([x_loc, z_loc]) as f64;
                self.mesh.push( (x_loc as f32, y_loc as f32, z_loc as f32) );
            }
        }
    }

    /// Will load a chunk if it's saved otherwise will generate it
    fn start_up(&mut self, noise: Arc<Fbm>) {
        self.generate(noise);
    }

    fn save(&self) {
        thread::sleep_ms(2);
    }

    fn load(&mut self) {
        thread::sleep_ms(2);
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
    noise:  Arc<Fbm>,
}

impl Terrain {
    /// Returns a new Terrain object that can be added to the engine
    pub fn new(point: Point, radius: isize) -> Terrain {
        let mut chunks = Vec::new();
        let origin: Coordinate = point.into();
        let noise = Arc::new(Fbm::new());

        for y in (-1 * radius)..=radius {
            for x in (-1 * radius)..=radius {
                let chunk = Arc::new(Mutex::new(
                    Chunk::new(x + origin.x, y + origin.y)
                ));
                chunks.push(chunk);
            }
        }

        Terrain { chunks, noise }
    }
}

impl EngineObject for Terrain {

    /// Loads or generates each chunk within render distance
    fn start_up(&mut self, core: &Core) {
        for chunk in &self.chunks {
            let chunk = chunk.clone();
            let noise = self.noise.clone();
            core.execute(move || {
                let mut chunk = chunk.lock().unwrap();
                chunk.start_up(noise);
            })
        }
    }

    /// Saves each chunk to a save folder
    fn save(&self, _core: &Core) {

    }

    /// Loads each chunk from a save folder
    fn load(&mut self, _core: &Core) {

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