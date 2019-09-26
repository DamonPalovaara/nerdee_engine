use glium::{Display};
use glutin::{self, EventsLoop, WindowBuilder, ContextBuilder, CursorState};

mod world;
use self::world::World;

const TITLE: &str = "Project Nightowl";

pub struct Game {
	window: Display,
	world:  World,
    center: (i32, i32),
} impl Game {
	pub fn new(events: &EventsLoop, size: (u32, u32)) -> Game {
		let window = {
            let window = WindowBuilder::new()
                .with_title(TITLE)
                .with_dimensions(size.0, size.1);
            let context = ContextBuilder::new().with_depth_buffer(24);
            Display::new(window, context, events).unwrap()
        };
        window.gl_window().set_cursor_state(CursorState::Hide).unwrap();
		Game {
			window: window.clone(),
			world:  World::new(&window.clone()),
            center: (size.0 as i32 / 2, size.1 as i32 / 2),
		}
	}
	pub fn update(&mut self, delta: u64) {

        let delta_f32 = delta as f32 / 1_000_000_000f32;
        //println!("{}", 1.0 / delta_f32);
        self.world.update(delta_f32);
        self.window.gl_window().set_cursor_position(self.center.0, self.center.1).unwrap();
	}
	pub fn draw(&mut self) {
        self.world.draw(&self.window);
	}
    pub fn resize(&mut self, size: (u32, u32)) {
        self.world.resize(size.0 as f32, size.1 as f32);
        self.center = (size.0 as i32 / 2, size.1 as i32 / 2);
    }
    pub fn forward_keyboard_events(&mut self, keycode: u32, pressed: glutin::ElementState) {
        self.world.player.controller.handle_keyboard_events(keycode, pressed);
    }
    pub fn forward_mouse_events(&mut self, position: (f64, f64)) {
        self.world.player.controller.handle_mouse_events(position.0 as f32, position.1 as f32);
    }
} impl Drop for Game {
    fn drop(&mut self) {
        println!("Dropping game struct!");
    }
} 