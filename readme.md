This is supposed to be the start of a game engine
* My goal was to create a realistic looking version of Minecraft
	* I created a terrain generator using simplex noise
	* I got a camera working with movement
	* Instead of doing infinite terrain I wanted to make an entire planet instead

Running the program
* As long as you have Rust install with cargo you should be able to use "cargo run" inside main directory
* Right now my terrain generator isn't "plugged" in so all you'll see is a cube
* I haven't implemented escape yet so you'll have to alt-tab to the command shell running it and press "ctrl+c" to exit (sorry)

Some notes to whoever might be reading this:
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
