use ash::{prelude::VkResult, vk, Entry};
use memtest_gpu_ecc::{Gpu, GIGABYTE, KILOBYTE, MEGABYTE};

#[cfg(target_env = "msvc")]
fn disable_msvcrt_debug_heap() {
    std::env::set_var("_NO_DEBUG_HEAP", "1");
}

fn main() -> VkResult<()> {
    // NVIDIA recommendation: disable msvcrt debug heap for debug builds
    if cfg!(debug_assertions) {
        disable_msvcrt_debug_heap();
    }

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

    // Initialize a new logical device
    gpu.new_device(1, vk::QueueFlags::TRANSFER); // copy commands need transfer queues

    // Define iterators for the sizes of each block that will be used to measure transfer speed
    let mut block_sizes_kb = (0..10).map(|n| (1 << n) * KILOBYTE);
    let mut block_sizes_mb = (0..10).map(|n| (1 << n) * MEGABYTE);
    let mut block_sizes_gb = (0..10).map(|n| (1 << n) * GIGABYTE);

    // Create the buffers!
    while let Some(block_kb) = block_sizes_kb.next() {
        gpu.new_buffer(block_kb);
    }
    while let Some(block_mb) = block_sizes_mb.next() {
        gpu.new_buffer(block_mb);
    }
    while let Some(block_gb) = block_sizes_gb.next() {
        gpu.new_buffer(block_gb);
    }

    Ok(())
}
