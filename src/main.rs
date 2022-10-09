use std::io::{self, prelude::*};
use ash::{prelude::VkResult, vk, Entry};

fn main() -> VkResult<()> {
    // Load the Vulkan library (if available on host environment)
    let entry = Entry::linked();

    // Select the highest API version supported by the host
    let api_version: u32 = match entry.try_enumerate_instance_version()? {
        Some(version) => version,
        None => vk::API_VERSION_1_0,
    };

    // Define application-specific metadata
    let application_info = vk::ApplicationInfo::builder()
        .api_version(api_version);

    // Define what properties an instance should have upon creation
    let instance_info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info);

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

    Ok(())
}
