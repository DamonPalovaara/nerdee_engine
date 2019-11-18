My objective is to create an engine that implements a procedural terrain generator that is tera-formable

What I have working so far:
* Basic Vulkan instance via Vulkano crate
* Basic terrain generator using multiple octaves of simplex noise
* .obj exporter for the generated terrain

What changes I want to make:
* I want to switch to using gfx-hal or rendy (high-level gfx-hal)

Future goals:
* Implement way to generate normals for the terrain
* Figure out way to "voxelize" terrain
* Figure out way to shape terrain into a planet