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

Tomorrows mission:
	- Figure out how to handle params
	- Implement input for camera
	- Reimplement Tarrian (consider starting from scratch)

Stretch mission:
	- Start shading terrain
	- Add water

Goal:
	- Make as modular as possible to make it easy to debug, maintainable, upgradable, portable, etc.
