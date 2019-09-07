Some notes to whoever else might be reading this:
	- I started this project over a year ago
	- At one point the Terrain was working and had direct lighting
	- I removed it so that I can focus on getting movement to work
	- If you look in the "unused" directory you'll find what I had for terrain generation
	- This isn't my best code, I've learned quite a bit since starting this project like:
		* Using Struct of arrays rather than vice versa
		* When and when not to use pointers
		* How to make my code more readable and modular
		* How to use enums for event handling (look at my Minesweeper project)
	- What this project shows is my willingness to self-learn
	- I want to reimplemant terrain but I'll have to spend some time reading this code to figure out where I left off
		* I didn't comment as well as I do now 



Roadmap:
	- Implement camera module
	- Implement draw function for terrain module
	- Use SIMD accelerated math library
		+ Write SIMD accelerated math library strongly compatable with Glium

Questions:
	- How should events be handled?
	- Can I use baked heightmap for collision detection?
		+ How would I update the heightmap?
			- Perhaps use GPU for initial heightmap but have CPU update it when terrain changes
		+ How would I handle caves?
		+ What resolution would I use?
			- Would I 2d lerp between points?

Stretch mission:
	- Start shading terrain
	- Add water

Goal:
	- Make as modular as possible to make it easy to debug, maintainable, upgradable, portable, etc.
