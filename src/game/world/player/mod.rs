mod camera;
mod controller;

use self::camera::Camera;
use self::controller::Controller;

use cgmath::{Vector3, Matrix4, Rad, Euler, Basis3, InnerSpace, Rotation, Zero};

const ACCELERATION: f32 = 0.05;
const DRAG:  f32 = 0.97;

// X and Y should scale with aspect ratio
const X_LOOK_SENS: f32 = -0.3;
const Y_LOOK_SENS: f32 = -0.5;

fn new_matrix() -> [[f32; 4]; 4] {
	[
		[1.0, 0.0, 0.0, 0.0],
		[0.0, 1.0, 0.0, 0.0],
		[0.0, 0.0, 1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0f32]
	]
}

pub struct Player {
	pub camera:      Camera,
	pub controller:  Controller,
	position:        Vector3<f32>,
	velocity:        Vector3<f32>,
	x_rot:           Rad<f32>,
	y_rot:           Rad<f32>,
	pub translation_matrix: [[f32; 4]; 4],
	pub    rotation_matrix: [[f32; 4]; 4],
} impl Player {
	pub fn new(width: f32, height: f32, fov: f32) -> Player {
		Player {
			camera:     Camera::new(width, height, fov),
			controller: Controller::new(width, height),
			position:   Vector3::new(0.0, 0.0, 10.0),
			velocity:   Vector3::new(0.0, 0.0, 0.0),
			x_rot: Rad(0.0),
			y_rot: Rad(0.0),
			translation_matrix: new_matrix(),
			rotation_matrix: new_matrix(),
		}
	}
	pub fn update(&mut self, delta_time: f32) {
		// X axis on mouse rotates the camera around the y axis
		self.y_rot += self.controller.x_axis * Y_LOOK_SENS;

		// Y axis on mouse rotates the camera around the x axis
		self.x_rot += self.controller.y_axis * X_LOOK_SENS;

		// Create the rotation matrix
		self.rotation_matrix = Matrix4::from(Euler {
			x: self.x_rot,
			y: self.y_rot,
			z: Rad(0.0)
		}).into();

		// Used to rotate the movement
		let rotate = Basis3::from(Euler {
    		x: Rad(0.0),
    		y: -self.y_rot,
    		z: Rad(0.0),
		});

		// Initialize a blank vector
		let mut offset = Vector3::new(0.0, 0.0, 0.0);

		// Figure out what move keys are being pressed and add to offset accordingly
		if self.controller.move_forward {
			offset.z -= 1.0;
		}
		if self.controller.move_backward {
			offset.z += 1.0;
		}
		if self.controller.move_left {
			offset.x += 1.0;
		}
		if self.controller.move_right {
			offset.x -= 1.0;
		}
		if self.controller.move_up {
			offset.y -= 1.0;
		}
		if self.controller.move_down {
			offset.y += 1.0;
		}

		// Normalize vector
		if !offset.is_zero() {
			self.velocity += rotate.rotate_vector( offset.normalize_to(ACCELERATION) );
		}

		// Add the velocity to the position
		self.position += self.velocity;			
		// Apply drag to the velocity
		self.velocity *= DRAG;
		
		//Create the translation matrix
		self.translation_matrix = Matrix4::from_translation( self.position ).into();
	}
}