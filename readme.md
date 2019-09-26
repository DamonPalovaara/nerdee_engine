This is the start of a game engine
* My goal was to create a realistic looking version of Minecraft
	* I created a terrain generator using simplex noise
	* I got a camera working with movement
	* Instead of doing infinite terrain I want to work on generating a whole planet

Running the program
* As long as you have Rust installed with cargo you should be able to use "cargo run" inside main directory
* Right now my terrain generator isn't "plugged" in so all you'll see is a cube
* I haven't implemented escape yet so you'll have to alt-tab to the command shell running it and press "ctrl+c" to exit (sorry)

TODO:
* ESCAPE KEY!
* ^^^ Need to refactor how input is handled (Handling escape key in player struct is awkward)
* Generate a sphere
* Come up with way to tessellate it (break it up into self-similar pieces)
* Use semplex noises to carve out terrain
* Implement more advanced techniques
	* Plate tectonics
	* Erosion
	* Glacier erosion
	* Caves
* Get an idea on how to reimplement the terrain module
* Begin work on implementing physics
	* Collision detection
		* It's possible to use a GPU to generate a heightmap
		* Treat heightmap as a 2D array that returns a height at point (x, y)
			* See visualization ^^
		* Interpolate in between points
	* Gravity
	* Orbiting planets
