use nerdee_engine::engine_core::*;
use nerdee_engine::terrain::Terrain;

fn main() {
    let mut engine = Engine::new(8);
    let location = (0.0, 0.0, 0.0);
    let render_distance = 2;
    let terrain = Box::new( Terrain::new(location, render_distance) );
    engine.add(terrain);
    engine.initialize();
    //engine.run_forever();
}