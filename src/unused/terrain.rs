use noise::{NoiseFn, OpenSimplex};

// Max chunk size is 256!
pub const CHUNK_SIZE_I32: i32   = 256;
pub const CHUNK_SIZE_F32: f32   = CHUNK_SIZE_I32 as f32;
pub const CHUNK_SQUARED:  usize = (CHUNK_SIZE_I32 * CHUNK_SIZE_I32) as usize;
pub const WORLD_HEIGHT:   f64   = 10.0;
// Used to convert actual location to quanatized location
// Spacing is the distance between grid point
pub const SPACING:        f32   = 0.1;
// The inverse of spacing for optimization reasons
pub const SPACING_RECP:   f32   = 1.0 / SPACING; 

pub fn normalize(a: &(f32, f32, f32)) -> (f32, f32, f32) {
	let temp = (a.0 * a.0, a.1 * a.1, a.2 * a.2);
	let len = temp.0 + temp.1 + temp.2;
	(temp.0 / len, temp.1 / len, temp.2 / len)
}

// BAKE THIS!
pub fn create_triangle_strip(chunk_size: u16) -> Vec<u16> {
    let mut temp: Vec<u16> = Vec::new();
    for j in 0..chunk_size - 1 {
        // Creates triangle strip for first row
        for i in 0..chunk_size {
            temp.push(i + j * chunk_size);
            temp.push(i + (j + 1) * chunk_size);
        }
        // First and last vertex needs to be repeated on every row 
        // ...except on first and last
        if j != chunk_size - 2 {
            // Add last vertex of current row
            temp.push((j + 2) * chunk_size - 1);
            // Add first vertex of next row
            temp.push((j + 1) * chunk_size);
        }
        //println!("{}", j)
    }
    temp
}

// 3D point
#[derive(Clone, Copy)]
pub struct Vertex {
	pub location: (f32, f32, f32),
	pub normal:   (f32, f32, f32)
} impl Vertex {
	// TODO: Figure out how world space maps to grid space
	pub fn to_pixel_location(&self) -> PixelLocation {
		PixelLocation {
			x: (self.location.0 * SPACING_RECP).round() as i32,
			y: (self.location.1 * SPACING_RECP).round() as i32,
			z: (self.location.2 * SPACING_RECP).round() as i32
		}
	}
	pub fn to_chunk_location(&self) -> ChunkLocation {
		ChunkLocation {
			x: (self.location.0 / CHUNK_SIZE_F32).floor() as i32,
			z: (self.location.2 / CHUNK_SIZE_F32).floor() as i32
		}
	}
}

// Make usable by OpenGl
implement_vertex!(Vertex, normal, location);

// Coordinate system for whole chunks on a grid
#[derive(PartialEq, Clone, Copy)]
pub struct ChunkLocation {
	pub x: i32,
	pub z: i32
}

// Location of pixel based on a grid,
// PixelLocation x SPACING = Vertex
pub struct PixelLocation {
	pub x: i32,
	pub y: i32,
	pub z: i32
} impl PixelLocation {
	pub fn to_vertex(&self) -> Vertex {
		Vertex {
			location: (
				self.x as f32 * SPACING,
				self.y as f32 * SPACING,
				self.z as f32 * SPACING
			),
			normal: (1.0, 1.0, 1.0),
		}
	}
}

// Contains the information for each chunk
pub struct Chunk(pub Vec<Vertex>);

// This struct is responsible for generating the terrain on request,
// It includes everything needed for terrain generation.
// Could use various different versions to create different dimensions.
pub struct TerrainGenerator {
	height_map: OpenSimplex,
	noise_scale: f64,
	percent_land: f64,
	land_offset: f64
} impl TerrainGenerator {

	pub fn new() -> TerrainGenerator {
		TerrainGenerator {
			height_map: OpenSimplex::new(),
			noise_scale: 0.01,
			// percent_land + land_offset must = 1.0!
			percent_land: 0.8,
			land_offset: 0.2
		}
	}

	pub fn load_chunk(&self, chunk_loc: ChunkLocation) -> Chunk {
		self.create_chunk(chunk_loc)
	}

	fn noise(&self, x: &f64, z: &f64) -> f64 {
		self.height_map.get([x * self.noise_scale, z * self.noise_scale]) * 
		WORLD_HEIGHT * self.percent_land - self.land_offset
	}

	/*
	pub fn save_chunk(&self, chunk_loc: ChunkLocation) -> Chunk {

	}
	*/

	pub fn create_chunk(&self, chunk_loc: ChunkLocation) -> Chunk {
		Chunk(
			{
				let mut chunk: Vec<Vertex> = Vec::with_capacity(CHUNK_SQUARED);

				let mut max = 0.0f64;

				// Maybe consider using a different coordinate system for "chunk space" vs world space
				// x and z seem to be confusing
				for z in chunk_loc.z * CHUNK_SIZE_I32..(chunk_loc.z + 1) * CHUNK_SIZE_I32 {
					for x in chunk_loc.x * CHUNK_SIZE_I32..(chunk_loc.x + 1) * CHUNK_SIZE_I32 {

						let x_loc = x as f64 * SPACING as f64;
						let z_loc = z as f64 * SPACING as f64;
						// Need to compute x and z before getting y
						let scale = 50.0;
						let y_loc = self.noise(&(x as f64), &(z as f64));

						if y_loc < max {max = y_loc;};

						chunk.push(Vertex {
							location: (
								x_loc as f32,								
								y_loc as f32,
								z_loc as f32
							),
							normal: normalize( &( x as f32 / CHUNK_SIZE_F32, 
												  y_loc as f32,
								                  z as f32 / CHUNK_SIZE_F32
								) 
							),
						})

					}
				}

				println!("{}", max);

				chunk
			}
		)
	}
}

// Only should need one per client
// Swaps out chunks if new world is loaded (nether, end, etc)
// Chunks loaded based on the current camera location (not the players)
pub struct TerrainStreamer {
	// How do I make this work with multiple dimensions?
	terrain:     TerrainGenerator,
	// ...
	render_dist: i32,
	// Current chunk placement position (in grid space)
	cur_loc: 	 ChunkLocation,
	// Current camera location (in grid space)
	cam_loc: 	 ChunkLocation,
	// Implement in a way where memory resizing doesn't happen if possible,
	// Consider wraping with pointer type (Maybe Rc<T>)
	chunk_locs:  Vec<ChunkLocation>,
	// Perhaps add location as part of chunk class
	chunks:      Vec<Chunk>
} impl TerrainStreamer {
	
	pub fn new(cam_loc: ChunkLocation, render_dist: i32) -> TerrainStreamer {
		// Loads all of the chunks initially
		let terrain: TerrainGenerator = TerrainGenerator::new();

		let capacity = (render_dist * 2 + 1).pow(2) as usize;

		let mut chunk_locs: Vec<ChunkLocation> = Vec::with_capacity(capacity);
		let mut chunks:     Vec<Chunk>         = Vec::with_capacity(capacity);

		let start = render_dist * -1;
		let end = render_dist + 1;

		for z in start..end {
			for x in start..end {

				let chunk_loc = ChunkLocation{x: x + cam_loc.x, z: z + cam_loc.z};

				chunk_locs.push(chunk_loc);
				chunks.push(terrain.load_chunk(chunk_loc));

			}			
		}
		
		TerrainStreamer {
			terrain: terrain,
			render_dist: render_dist,
			cur_loc: cam_loc.clone(),
			cam_loc: cam_loc.clone(),
			chunk_locs: chunk_locs,
			chunks: chunks,
		}		
	}

	pub fn update(&mut self, cam_loc: ChunkLocation, delta_time: f64) {
		// ...
		// self.age_terrain(delta_time);
		// ...
		if self.cam_loc != cam_loc {
			self.cam_loc = cam_loc;
			self.update_chunks();
		}
	}

	pub fn update_chunks(&mut self) {
		// Updates the chunks to a new center grid position
		// ...
		// for position in chunk_locs:
		//    is position outside of new render distance?
		//        save chunk then overwrite data with a new chunk if so
	}

	pub fn update_render_distance() {
		// Logic involved for either increasing or decreasing render distance
	}
}
