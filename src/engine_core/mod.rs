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
};
use vulkano_win::VkSurfaceBuild;

#[derive(Debug)]
pub struct EngineCore {

	instance: Arc<Instance>,
	physical: usize,
	surface:  Arc<Surface<Window>>,
	device:   Arc<Device>,

}

impl EngineCore {

	pub fn new() -> EngineCore {

		let instance = Self::create_instance();
		let physical = Self::pick_physical_device(&instance);
		let surface  = Self::create_surface(&instance);
		let device   = Self::create_device(&instance, physical);

		EngineCore {
			instance: instance,
			physical: physical,
			surface:  surface,
			device:   device,
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

	fn pick_physical_device(instance: &Arc<Instance>) -> usize {
		for physical_device in PhysicalDevice::enumerate(&instance) {
		    println!("Available device: {}", physical_device.name());
		}
		return 0;
    }

    fn create_device(instance: &Arc<Instance>, physical_idx: usize) -> Arc<Device> {

    	let physical_device = PhysicalDevice::from_index(&instance, physical_idx).unwrap();

    	let queue_family = physical_device.queue_families().next().unwrap();
    	let features = Features::none();
    	let ext = DeviceExtensions::none();

    	match Device::new(physical_device, &features, &ext, Some((queue_family, 1.0))) {
        	Ok(d) => d.0,
        	Err(err) => panic!("Couldn't build device: {:?}", err)
    	}
	}


	pub fn debug_print(&self) {
		println!("Instance: {:?}",    self.instance);
		println!("Physical ID: {:?}", self.physical);
		println!("Surface: {:?}",     self.surface);
		println!("Device: {:?}",      self.device);
	}
    
}