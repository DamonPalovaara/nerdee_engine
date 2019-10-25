#[macro_use]
extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

mod cube;
mod engine_core;

//use cube::Cube;
use engine_core::EngineCore;
use winit::{WindowEvent, DeviceEvent};

fn main() {
	
	let mut engine_core = EngineCore::new();
	engine_core.run_forever();
}