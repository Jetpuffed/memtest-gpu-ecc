use ash::{prelude::VkResult, vk, Device, Instance};

pub struct Gpu<'a> {
    instance: &'a Instance,
    handle: &'a vk::PhysicalDevice,
    devices: Vec<Device>,
}

impl<'a> Gpu<'a> {
    pub fn new(instance: &'a Instance, handle: &'a vk::PhysicalDevice) -> Self {
        Self {
            instance,
            handle,
            devices: Vec::new(),
        }
    }

    /// Creates a new logical device and appends it to the `devices` vec. Returns a reference to the newly created device.
    pub fn new_device(&mut self, create_info: &vk::DeviceCreateInfo) -> VkResult<&Device> {
        let device = unsafe {
            self.instance
                .create_device(*self.handle, create_info, None)?
        };
        self.devices.push(device);
        Ok(&self.devices[self.devices.len() - 1])
    }
}
