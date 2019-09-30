// Used for creating a window and receiving events
// docs.rs/winit/0.19.3/winit/index.html
extern crate winit;

// Event loop stuff
// docs.rs/winit/0.19.3/winit/struct.EventsLoop.html
use winit::{Event, WindowEvent, DeviceEvent};

fn main() {
	// Creates the game struct then runs it
	let game = Game::new();
	println!("{:?}", game);
}

#[derive(Debug)]
struct Game {

	running: bool, // Used to break out of game-loop

} 

impl Game {

	// Init struct
	fn new() -> Game {
		Game {
			running: true,
		}
	}
	
	// Run the game, note that this method runs until game is closed
	fn run(&mut self) {
		let mut events = winit::EventsLoop::new();
		let _window = winit::Window::new(&events).unwrap();

		// The game loop
		while self.running {
			// Sends every new event into the handle_event method
			// Think of this as for event in events { self.handle_event(event) }
			events.poll_events( |event| self.handle_event(event) );
			self.update();
			self.draw();
		}
	}

	// Handles each event
	fn handle_event(&mut self, event: winit::Event) {

		// This is a match block, it's job is to figure out what event was passed
		// and do different things for each event
		// This is a nested match block, first part sorts between window and device events
		// doc.rust-lang.org/rust-by-example/flow_control/match.html
		// docs.rs/winit/0.19.3/winit/enum.Event.html
		match event {

			// These are events associated with the window
			// docs.rs/winit/0.19.3/winit/enum.WindowEvent.html
			Event::WindowEvent { event, .. } => match event {

				WindowEvent::CloseRequested => self.running = false,
				// ...
				_ => (), // Default statement

			}

			// These are events associated with the mouse
			// docs.rs/winit/0.19.3/winit/enum.DeviceEvent.html
			Event::DeviceEvent { event, .. } => match event {

				DeviceEvent::MouseMotion{delta} => println!("{}, {}", delta.0, delta.1),
				_ => (),

			}

			// Default statement
			_ => (),

		}
	}

	// Update all of the game logic
	fn update(&mut self) {
		// Do update stuff here
	}
	// Renders to screen
	// * Note that self isn't mut, update is responsible for changing values
	fn draw(& self) {
		// Do render stuff here
	}
}

// Called once game leaves scope
impl Drop for Game {
	fn drop(&mut self) {
		println!("Game has closed");
	}
}