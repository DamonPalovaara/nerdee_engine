use glutin;
use cgmath::{Rad};

pub struct Controller {
	half_width:  f32,
	half_height: f32,

	pub move_forward:  bool,
	pub move_backward: bool,
	pub move_left:     bool,
	pub move_right:    bool,
	pub move_up:       bool,
	pub move_down:     bool,

	pub x_axis: Rad<f32>,
	pub y_axis: Rad<f32>,
} impl Controller {
	pub fn new(width: f32, height: f32) -> Controller {
		Controller {
			half_width:  width  * 0.5,
			half_height: height * 0.5,

			move_forward:  false,
			move_backward: false,
			move_left:     false,
			move_right:    false,
			move_up   :    false,
			move_down :    false,

			x_axis: Rad(0.0f32),
			y_axis: Rad(0.0f32),
		}
	}
	// Call this when ever the state of the keys are uncertain such as:
	// ... Controller creation
	// ... Reactivation of the window
	pub fn reset_key_states() {

	}
	pub fn handle_keyboard_events(&mut self, keycode: u32, pressed: glutin::ElementState) {

		// True if pressed\
		let state = match pressed {
			glutin::ElementState::Pressed => true,
			_ => false
		};

		//println!("{}", keycode);

		// Consider getting rid of unneeded variable declarations
		match keycode {
			// W key
			17 => self.move_forward  = state,
			// A key
			30 => self.move_left     = state,
			// S key
			31 => self.move_backward = state,
			// D key
			32 => self.move_right    = state,
			// Left Shift key
			42 => self.move_down     = state,
			// Space bar
			57 => self.move_up       = state,
			// Other
			_  => (),
			//_  => println!("{}", keycode),
		}
	}
	pub fn handle_mouse_events(&mut self, x: f32, y: f32) {
		self.x_axis = Rad( (x - self.half_width ) / self.half_width  );
		self.y_axis = Rad( (y - self.half_height) / self.half_height );
		//println!("x: {:?}, y: {:?}", self.x_axis, self.y_axis);
	}
	pub fn resize(&mut self, width: f32, height: f32) {
		self.half_width  = width  / 2.0;
		self.half_height = height / 2.0;
	}
}