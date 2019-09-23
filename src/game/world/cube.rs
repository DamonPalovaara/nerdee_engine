use Vertex;
use glium;
use glium::{Display, VertexBuffer, IndexBuffer, Program, DrawParameters, Frame, Surface};

// The vertices of the cube
const CUBE_VERTICES: [Vertex; 8] = [
	Vertex { position: [-1.0, -1.0, -1.0] },
	Vertex { position: [-1.0,  1.0, -1.0] },
	Vertex { position: [-1.0, -1.0,  1.0] },
	Vertex { position: [-1.0,  1.0,  1.0] },
	Vertex { position: [ 1.0, -1.0, -1.0] },
	Vertex { position: [ 1.0,  1.0, -1.0] },
	Vertex { position: [ 1.0, -1.0,  1.0] },
	Vertex { position: [ 1.0,  1.0,  1.0] }
];

// The lines that make up the cube
// The numbers are indexes to the array above ^^^
const CUBE_INDICES: [u16; 36] = [	
	// Left
	0, 1, 2,
	1, 3, 2,
	// Top
	3, 1, 7,
	7, 1, 5,
	// Right
	6, 7, 4,
	7, 4, 5,
	// Bottom
	0, 2, 4,
	2, 4, 6,
	// Front
	2, 3, 7,
	2, 7, 6,
	// Back
	0, 1, 5,
	0, 5, 4
];

// Consider saving shaders as seperate files and loading them instead

// Vertex shader for the cube
const CUBE_VERTEX_SHADER: &str = r#"
	#version 150

	in vec3 position;

	out vec3 v_position;

	uniform mat4 perspective;
	uniform mat4 rotation;
	uniform mat4 translation;
	uniform mat4 model;

	void main() {
		gl_Position = perspective * rotation * translation * model * vec4(position, 1.0);
		v_position = gl_Position.xyz / gl_Position.w;
	}
"#;

// Fragment shader for the cube
const CUBE_FRAGMENT_SHADER: &str = r#"
	#version 150

	in vec3 v_position;

	out vec4 color;

	const vec3 diffuse_color = vec3(0.6, 0.0, 0.0);

	void main() {
		color = vec4(diffuse_color, 1.0);
	}
"#;


pub struct Cube {
	pub positions: VertexBuffer<Vertex>,
	pub indices:   IndexBuffer<u16>,
	pub program:   Program,
	pub model_matrix:    [[f32; 4]; 4],
} impl Cube {
	pub fn new(display: &Display) -> Cube {
		Cube {
			positions : VertexBuffer::new(display, &CUBE_VERTICES).unwrap(),
			indices   : IndexBuffer::new(display, 
										 glium::index::PrimitiveType::TrianglesList, 
										 &CUBE_INDICES).unwrap(),
			program   : Program::from_source(display, CUBE_VERTEX_SHADER,
											 CUBE_FRAGMENT_SHADER, None).unwrap(),
			model_matrix    : [
            	[1.0, 0.0, 0.0, 0.0],
            	[0.0, 1.0, 0.0, 0.0],
            	[0.0, 0.0, 1.0, 0.0],
            	[0.0, 0.0, 0.0, 1.0f32] 
        	],
		}
	}

	// This needs to be cleaned up
	pub fn draw(& self, 
		mut target:  Frame,
		translation: [[f32; 4]; 4], // raw 4x4 matrix
		rotation:    [[f32; 4]; 4], 
		perspective: [[f32; 4]; 4], 
		params: DrawParameters) -> Frame 
	{
		 // The actual draw call
        target.draw(
        	& self.positions,
        	& self.indices,
        	& self.program,
        	// Combine the matrices into one on CPU to save on GPU overhead
        	& uniform! {translation: translation, rotation: rotation, perspective: perspective, model: self.model_matrix},
        	& params
        ).unwrap();

        target
	}
}