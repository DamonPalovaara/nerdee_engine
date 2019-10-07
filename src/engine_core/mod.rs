use vulkano::sync::SharingMode;
use vulkano::swapchain::Swapchain;
use vulkano::swapchain::PresentMode;
use vulkano::swapchain::CompositeAlpha;
use vulkano::image::SwapchainImage;
use vulkano::image::ImageUsage;
use winit::Window;
use winit::EventsLoop;
use winit::WindowBuilder;


use vulkano::swapchain::Surface;
use std::sync::Arc;
use vulkano::instance::{
	Instance, 
	InstanceExtensions,
	Version,
	ApplicationInfo,
	PhysicalDevice,
};
use vulkano::device::{
	Device,
	DeviceExtensions,
	Features,
	Queue,
};
use vulkano_win::VkSurfaceBuild;

pub struct EngineCore {

	instance: Arc<Instance>,
	physical: usize,
	surface:  Arc<Surface<Window>>,
	device:   Arc<Device>,
	graphics_queue: Arc<Queue>,
	present_queue:  Arc<Queue>,
	swap_chain: Arc<Swapchain<Window>>,
	swap_chain_images: Vec<Arc<SwapchainImage<Window>>>,

}

impl EngineCore {

	pub fn new() -> EngineCore {

		let instance = Self::create_instance();
		let physical = Self::pick_physical_device(&instance);
		let surface  = Self::create_surface(&instance);
		let (device, graphics_queue, present_queue) = Self::create_device(&instance, physical);
		let (swap_chain, swap_chain_images) = Self::create_swap_chain(&surface, &device, &present_queue, None);

		EngineCore {
			instance: instance,
			physical: physical,
			surface:  surface,

			device:         device,
			graphics_queue: graphics_queue,
			present_queue:  present_queue,

			swap_chain:        swap_chain,
			swap_chain_images: swap_chain_images,
		}
	}

	fn create_instance() -> Arc<Instance> {

		// TODO load app_info from txt file
		let app_info = ApplicationInfo {
			application_name:    Some("NerDee Engine".into()),
			application_version: Some(Version { major: 0, minor: 1, patch: 0 }),
			engine_name:         Some("NerDee Engine".into()),
			engine_version:      Some(Version { major: 0, minor: 1, patch: 0 }),
		};

		let extensions: InstanceExtensions = vulkano_win::required_extensions();

		Instance::new(Some(&app_info), &extensions, None).expect("failed to create Vulkan instance")
	}

	fn create_surface(instance: &Arc<Instance>) -> Arc<Surface<Window>> {
		let events_loop = EventsLoop::new();
		match WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()) {
			Ok(surface) => surface,
			Err(err)    => panic!("Couldn't create surface: {:?}", err)
		}
	}

	// TODO: Allow user selection of device
	fn pick_physical_device(instance: &Arc<Instance>) -> usize {
		for physical_device in PhysicalDevice::enumerate(&instance) {
			println!("Available device: {}", physical_device.name());
		}
		return 0;
	}

	fn create_device(instance: &Arc<Instance>, physical_idx: usize) -> (Arc<Device>, Arc<Queue>, Arc<Queue>) {

		let physical_device = PhysicalDevice::from_index(&instance, physical_idx).unwrap();

		let queue_family = physical_device.queue_families().find(|&q| q.supports_graphics()).unwrap();
		let features = Features::none();
		let ext = DeviceExtensions {
			khr_swapchain: true,
			.. DeviceExtensions::none()
		};

		let (device, mut queues) = match Device::new(physical_device, &features, &ext, Some((queue_family, 1.0))) {
			Ok(device) => device,
			Err(err)   => panic!("Couldn't build device: {:?}", err)
		};

		let graphics_queue = queues.next().unwrap();
		let present_queue  = queues.next().unwrap_or_else(|| graphics_queue.clone());

		(device, graphics_queue, present_queue)

	}

	// TODO: Handle errors and implement way of choosing composite, color format, present mode, etc
	fn create_swap_chain(
		surface: &Arc<Surface<Window>>,
		device: &Arc<Device>,
		present_queue:  &Arc<Queue>,
		old_swapchain: Option<Arc<Swapchain<Window>>>,
	) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>) {
		//let physical_device = PhysicalDevice::from_index(physical_idx);
		let caps = surface.capabilities(device.physical_device()).unwrap();
		let dimensions = caps.current_extent.unwrap_or([1280, 720]);
		
		// Implement safe way of getting buffer count
		let buffers_count = 2;

		let transform = caps.current_transform;

		let (format, _color_space) = caps.supported_formats[0];

		let usage = ImageUsage {
			color_attachment: true,
			.. ImageUsage::none()
		};

		let sharing_mode = SharingMode::Exclusive(present_queue.family().id());

		let (swapchain, buffers) = Swapchain::new(
			device.clone(),
			surface.clone(),
			buffers_count,
			format,
			dimensions,
			1,
			usage,
			sharing_mode,
			transform,
			CompositeAlpha::Opaque,
			PresentMode::Fifo,
			true,
			old_swapchain.as_ref()
		).unwrap();

		(swapchain, buffers)
	}



	pub fn debug_print(&self) {
		println!("Instance: {:?}",    self.instance);
		println!("Physical ID: {:?}", self.physical);
		println!("Surface: {:?}",     self.surface);
		println!("Device: {:?}",      self.device);
		println!("Graphics queue: {:?}", self.graphics_queue);
		println!("Present queue: {:?}",  self.present_queue);
	}
	
}