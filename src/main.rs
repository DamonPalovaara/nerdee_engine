extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

mod cube;
mod engine_core;

use cube::Cube;
use engine_core::EngineCore;



fn main() {

    let engine_core = EngineCore::new();
    engine_core.debug_print();

}