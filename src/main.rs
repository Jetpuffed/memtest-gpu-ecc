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

    Ok(())
}
