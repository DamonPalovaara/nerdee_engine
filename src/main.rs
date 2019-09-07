extern crate noise;
#[macro_use]
extern crate glium;
extern crate time;
extern crate cgmath;

use glium::glutin::{self, EventsLoop};

mod game;
use game::Game;

const TAU_F32: f32 = std::f32::consts::PI;
const FRAME_RATE: f64 = 120.0;
const FRAME_GAP_NS: u64 = ((1.0 / FRAME_RATE) * 1_000_000_000.0) as u64;

#[derive(Copy, Clone)]
pub struct Vertex {
	position: [f32; 3],
}

implement_vertex!(Vertex, position);

fn main() {	
	setup();
}

// Setups everything to run
fn setup() {
	let events = EventsLoop::new();
	let mut game = Game::new(&events, (1280, 720));
	game_loop(events, game);
}

fn exit_program() {
	println!("Program is exiting!");
}

// The heart beat
fn game_loop(mut events: EventsLoop, mut game: Game) {
	// Init the time variables
	let mut time_now = time::precise_time_ns();
	let mut time_last = time_now.clone();
	let mut time_delta = 0;
	// True when program is ready to exit
	// Either use exit_program() or the drop trait for anything that needs to be done after exit (e.g. saving data)
	let mut exit = false;

	// The actual game loop
	while !exit {

		// Polling events
		events.poll_events(|event| {
			match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed                    => exit = true,
                    glutin::WindowEvent::Resized(x, y)             => game.resize((x, y)),
                    glutin::WindowEvent::KeyboardInput{ input, ..} => game.forward_keyboard_events(input.scancode, input.state),
                    glutin::WindowEvent::CursorMoved{ position,..} => game.forward_mouse_events(position),
                    _ => ()
                },
                _ => ()
            };
		});

		// Update the game
		game.update(time_delta);
		// Draw the game
		game.draw();		

		// Update time variables
		time_last = time_now;
		time_now = time::precise_time_ns();
		// Enforce frame cap
		while time_now - time_last < FRAME_GAP_NS { time_now = time::precise_time_ns() };
		// Consider moving this else where
		time_delta = time_now - time_last;
	}
	// Call to exit program function after the while loop breaks
	exit_program();
}