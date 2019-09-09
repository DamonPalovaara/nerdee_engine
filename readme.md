This is the start of a game engine
* My goal was to create a realistic looking version of Minecraft
	* I created a terrain generator using simplex noise
	* I got a camera working with movement
	* Instead of doing infinite terrain I wanted to make an entire planet instead

Running the program
* As long as you have Rust install with cargo you should be able to use "cargo run" inside main directory
* Right now my terrain generator isn't "plugged" in so all you'll see is a cube
* I haven't implemented escape yet so you'll have to alt-tab to the command shell running it and press "ctrl+c" to exit (sorry)

Some notes to anybody reading this:
* I started this project over a year ago
* At one point the Terrain was working and had direct lighting
* I removed it so that I can focus on getting movement to work
* If you look in the "unused" directory you'll find what I had for terrain generation
* This isn't my best code, I've learned quite a bit since starting this project like:
	* Using Struct of arrays rather than vice versa
	* When and when not to use pointers
	* How to make my code more readable and modular
	* How to use enums for event handling (look at my Minesweeper project)
* What this project shows is my willingness to self-learn
* I want to reimplement terrain but I'll have to spend some time reading this code to figure out where I left off
	* I didn't comment as well as I do now 

TODO:
* Read through code and comment everything
* Get an idea on how to reimplement the terrain module
* Consider programming a "planet" module (in place of terrain)
	* Only surface to be generated
	* Ground only generated when needed (jit generation)
	* Materials will have a "hardness" value used for erosion simulation
	* Figure out an acurate way to simulate water
		* Create inland lakes
		* Rivers that flow from high to low
		* Erode the terrain based on water flow rate, hardness of material, and age
			* Be able to form canyons procedurally
	* How would I tesalate a planet
* Begin work on implementing physics
	* Collision detection
		* It's possible to use a GPU to generate a heightmap
		* Treat heightmap as 2D array that returns a height at point (x, y)
			* See visualizations ^^
		* Interpolate inbetween points
	* Gravity
		* If done on the planet scale I'll have to keep track of orientation and where to fall
			* Could have each chunk generate a tangent plane to the center point
			* See visualizations ^^ 
	* Orbiting planets