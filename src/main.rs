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
	
	let mut running = true;	
	while running {
		
		engine_core.events_loop.poll_events(|event| {

			match event {

				winit::Event::WindowEvent { event, .. } => match event {
					WindowEvent::CloseRequested => running = false,
					_ => ()
				},

				winit::Event::DeviceEvent { event, .. } => match event {
					DeviceEvent::MouseMotion{ delta } => println!("{:?}", delta),
					_ => (),
				},

				_ => (),

			};

		});
		
	}
}