extern crate read_input;
extern crate vulkano;
extern crate image;

use std::sync::Arc;

use read_input::prelude::*;

use image::{ImageBuffer, Rgba};

use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use vulkano::device  ::{Device, DeviceExtensions, Features};
use vulkano::buffer  ::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBuffer};
use vulkano::sync::GpuFuture;
use vulkano::pipeline::ComputePipeline;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::format::{ClearValue, Format};
use vulkano::image::Dimensions;
use vulkano::image::StorageImage;

fn main() {

	// Get a physical device
	let instance = Instance::new(None, &InstanceExtensions::none(), None)
		.expect("Failed to create instance");
	let devices: Vec<PhysicalDevice> = PhysicalDevice::enumerate(&instance).collect();
	for device in &devices {
		println!("{}: {}", device.index(), device.name());
	}
	// Have user select which device
	let physical = devices[
		input()
			.inside_err(0..devices.len(), "Index not in range")
			.repeat_msg("Select a device: ")
			.err("Input must be a valid index (e.g. \"0\")").get()
	];
	println!("Using {}", physical.name());

	// Create a queue family that supports graphics
	let queue_family = physical.queue_families()
		.find(|&q| q.supports_graphics())
		.expect("Couldn't find a graphical queue family");

	// Create the device and queues iter
	let (device, mut queues) = {
		Device::new(
			physical, 
			&Features::none(), 
			&DeviceExtensions::none(),
			[(queue_family, 0.5)].iter().cloned()
		).expect("Failed to create device")
	};

	// Select a queue to use
	let queue = queues.next().unwrap();

	let image = StorageImage::new(
		device.clone(),
		Dimensions::Dim2d {
			width: 1024,
			height: 1024
		},
		Format::R8G8B8A8Unorm,
		Some(queue.family())
	).unwrap();

	let buf = CpuAccessibleBuffer::from_iter(
		device.clone(),
		BufferUsage::all(),
		(0..1024*1024 * 4).map(|_| 0u8)
	).expect("Failed to create buffer");

	let command_buffer = AutoCommandBufferBuilder::new(
		device.clone(),
		queue.family()
	).unwrap().clear_color_image(
		image.clone(),
		ClearValue::Float([0.0, 0.0, 1.0, 1.0])
	).unwrap().copy_image_to_buffer(
		image.clone(),
		buf.clone()
	).unwrap().build().unwrap();

	let finished = command_buffer.execute(queue.clone()).unwrap();
	finished.then_signal_fence_and_flush().unwrap().wait(None).unwrap();

	let buffer_content = buf.read().unwrap();
	let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
	image.save("image.png").unwrap();

	println!("Everything succeeded!");

}