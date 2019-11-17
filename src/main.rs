#[macro_use]
extern crate vulkano;
extern crate vulkano_win;
extern crate winit;
extern crate noise;
extern crate time;

//mod cube;
//mod engine_core;

//use cube::Cube;
//use engine_core::EngineCore;

mod terrain;
use terrain::Terrain;
use time::{Duration, PreciseTime};

fn main() {
	let start = PreciseTime::now();
	let terrain = Terrain::new(1000);
	terrain.write();
	println!("{}", start.to(PreciseTime::now()).num_seconds())
}