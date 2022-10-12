use ash::{prelude::VkResult, vk, Entry};
use memtest_gpu_ecc::Gpu;

fn main() -> VkResult<()> {
    // Load the Vulkan library (if available on host environment)
    let entry = Entry::linked();

    // Select the highest API version supported by the host
    let api_version: u32 = match entry.try_enumerate_instance_version()? {
        Some(version) => version,
        None => vk::API_VERSION_1_0,
    };

    // Define application-specific metadata
    let application_info = vk::ApplicationInfo::builder().api_version(api_version);

    // Define what properties an instance should have upon creation
    let instance_info = vk::InstanceCreateInfo::builder().application_info(&application_info);

    // Initialize a new instance to be used for the remainder of the program's run time
    let instance = unsafe { entry.create_instance(&instance_info, None)? };

    // Retrieve a list of all physical devices present on host environment
    let physical_devices = unsafe { instance.enumerate_physical_devices()? };

    // Select the primary physical device, or prompt user to select desired device if more than one are found
    let physical_device = if physical_devices.len() > 1 {
        unimplemented!()
    } else {
        physical_devices[0]
    };

    // Simplify GPU operations with a wrapper around this physical device
    let mut gpu = Gpu::new(&instance, &physical_device);

    // Find a queue family that supports transfer operations
    let queue_family_index = gpu
        .properties()
        .queue_family_properties()
        .iter()
        .position(|x| (x.queue_flags.as_raw() & vk::QueueFlags::TRANSFER.as_raw()) != 0)
        .expect("unable to find queue family with transfer capabilities");
    let queue_priorities = [1.0]; // array length is equal to # of queues to be requested

    // Define what queue(s) a device should have access to upon creation
    let queue_create_infos = [vk::DeviceQueueCreateInfo::builder()
        .queue_family_index(queue_family_index as u32)
        .queue_priorities(&queue_priorities)
        .build()];

    // Build a logical device with all supported features enabled
    let enabled_features = *gpu.properties().physical_device_features();
    let device_info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_create_infos)
        .enabled_features(&enabled_features);
    gpu.new_device(&device_info)?;

    Ok(())
}
