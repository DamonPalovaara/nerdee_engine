

#[derive(Debug, Copy, Clone)]
pub struct Cube {
	x: f32,
}

impl Cube {
	pub fn new(x: f32) -> Cube {
		Cube {
			x: x,
		}
	}

	pub fn print(&self) {
		println!("x: {}", self.x);
	}
}