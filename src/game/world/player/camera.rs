const ZNEAR: f32 = 0.1;
const ZFAR:  f32 = 1024.0;
const A:     f32 = (ZFAR + ZNEAR) / (ZFAR - ZNEAR);
const B:     f32 = -(2.0 * ZFAR * ZNEAR) / (ZFAR - ZNEAR);

fn get_perspective(width: f32, height: f32, fov_matrix: f32) -> [[f32; 4]; 4] {

    let aspect_ratio = height / width ;

    [
        [fov_matrix * aspect_ratio,        0.0, 0.0, 0.0],
        [                      0.0, fov_matrix, 0.0, 0.0],
        [                      0.0,        0.0,   A, 1.0],
        [                      0.0,        0.0,   B, 0.0],
    ]
}

fn degrees_to_matrix(fov_degrees: f32) -> f32 {
	1.0 / (fov_degrees.to_radians() / 2.0).tan()
}

pub struct Camera {
	width:       f32,
	height:      f32,
	fov_degrees: f32,
	fov_matrix:  f32,
    // Our matrices for OpenGl
	pub perspective: [[f32; 4]; 4],
} impl Camera {
    pub fn new(width: f32, height: f32, fov_degrees: f32) -> Camera {
    	let fov_matrix = degrees_to_matrix(fov_degrees);
        Camera {
        	width:       width,
        	height:      height,
        	fov_degrees: fov_degrees,
        	fov_matrix:  fov_matrix,
        	perspective: get_perspective(width, height, fov_matrix),
        }
    }
    pub fn resize(&mut self, width: f32, height: f32) {
    	self.perspective = get_perspective(width, height, self.fov_matrix);
    }
    fn update_fov(&mut self, fov_degrees: f32) {
    	self.fov_degrees = fov_degrees;
    	// Converts it to the value needed for the perspective matrix
    	self.fov_matrix = degrees_to_matrix(fov_degrees);
    	self.perspective = get_perspective(self.width, self.height, self.fov_matrix);
    }
}