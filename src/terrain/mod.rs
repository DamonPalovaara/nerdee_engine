use noise::{NoiseFn, OpenSimplex};
use std::fs::File;
use std::io::prelude::*;

pub struct Terrain {
    noise: OpenSimplex,
    chunks: Vec<Chunk>,
}

impl Terrain {
    pub fn new(size: usize) -> Terrain {
        let noise = OpenSimplex::new();
        let chunks = vec![Chunk::new(size, noise)];

        Terrain { noise, chunks }
    }

    pub fn print(&self) {
        for i in 0..self.chunks.len() {
            self.chunks[i].print();
        }
    }

    pub fn write(&self) {
        for i in 0..self.chunks.len() {
            self.chunks[i].write();
        }
    }
}

struct Chunk {
    size:   usize,
    points: Vec<f64>,
}

impl Chunk {
    fn new(size: usize, noise: OpenSimplex) -> Chunk {        
        let mut points = Vec::with_capacity(size * size);

        for y in 0..size {
            for x in 0..size {
                points.push(noise.get([x as f64, y as f64]));
            }
        }

        Chunk {
            size,
            points 
        }
    }

    fn print(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                print!("{:.2} ", self.points[y * self.size + x]);
            }
            println!("");
        }
    }

    // Writes chunk to .obj file
    // TODO: Implement an object format that supports binary
    // wikipedia.org/wiki/PLY_(file_format)
    // wikipedia.org/wiki/STL_(file_format)
    fn write(&self) -> std::io::Result<()> {
        let mut file = File::create("chunk.obj")?;
        let mut buffer = "".to_string();
        for i in 0..(self.size * self.size) {
            buffer.push_str(&format!(
                "v {:.2}, {:.2}, {:.2}\n",
                0.0, 0.0, self.points[i]
            ));
        }
        file.write(buffer.as_bytes())?;
        Ok(())
    }
}
