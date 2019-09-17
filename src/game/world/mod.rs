mod player; 
mod cube;

use glium::{self, Display, Surface, DrawParameters}; 
use self::player::Player;                             
use self::cube::Cube;

pub struct World {
	pub player: Player, // Why is this public?
	pub cube:   Cube,
	// terrain
} impl World {
	pub fn new(display: &Display) -> World {
		let (width, height) = display.get_framebuffer_dimensions();
		World {
			player: Player::new(width as f32, height as f32, 50.0),

			cube:   Cube::new(display),
		}
	}
	pub fn update(&mut self, delta_time: f32) {
		self.player.update(delta_time);
	}
	pub fn draw(&self, window: &Display) {
		// Creates fram to draw on
		let mut target = window.draw();
		// Clears the buffer
		target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

		// Rendering Params
		let params = DrawParameters{
			// Enable depth testing
    		depth: glium::Depth {
    			test: glium::draw_parameters::DepthTest::IfLess,
    			write: true,
    			.. Default::default()
    		},
    		// Enable Line render
    		polygon_mode: glium::draw_parameters::PolygonMode::Line,
    		.. Default::default()
        };

        // All objects with a draw method need to return target allowing me to chain the draw calls together (needed because Frame doesn't implement clone and it's cleaner)
        // Consider adding a custom draw function to frame instead which takes in a custom object type (target.draw(cube).draw(terrain)...)
    	self.cube.draw(target, self.player.translation_matrix, self.player.rotation_matrix, self.player.camera.perspective, params).finish().unwrap();
	}
	pub fn resize(&mut self, width: f32, height: f32) {
		// A bit hackish, consider refactoring
		self.player.camera    .resize(width, height);
		self.player.controller.resize(width, height);
	}
}