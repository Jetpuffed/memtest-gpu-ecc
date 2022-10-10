use std::collections::HashMap;
use ash::{prelude::VkResult, vk, Device, Instance};

pub struct Gpu<'a> {
    instance: &'a Instance,
    handle: &'a vk::PhysicalDevice,
    devices: Vec<Device>,
    allocations: HashMap<vk::Device, Vec<vk::DeviceMemory>>,
    buffers: HashMap<vk::Device, Vec<vk::Buffer>>,
}

impl<'a> Gpu<'a> {
    pub fn new(instance: &'a Instance, handle: &'a vk::PhysicalDevice) -> Self {
        Self {
            instance,
            handle,
            devices: Vec::new(),
            allocations: HashMap::new(),
            buffers: HashMap::new(),
        }
    }

    /// Creates a new logical device and appends it to the `devices` vec.
    pub fn new_device(&mut self, create_info: &vk::DeviceCreateInfo) -> VkResult<()> {
        let device = unsafe {
            self.instance
                .create_device(*self.handle, create_info, None)?
        };
        self.devices.push(device);
        let device = self.devices[self.devices.len() - 1].handle();
        self.allocations.insert(device, Vec::new());
        self.buffers.insert(device, Vec::new());
        Ok(())
    }

    pub fn new_allocation(&mut self, idx: usize, create_info: &vk::MemoryAllocateInfo) -> VkResult<()> {
        let device = &self.devices[idx];
        let allocation = unsafe { device.allocate_memory(create_info, None)? };
        self.allocations.entry(device.handle()).and_modify(|v| v.push(allocation));
        Ok(())
    }

    pub fn new_buffer(&mut self, idx: usize, create_info: &vk::BufferCreateInfo) -> VkResult<()> {
        let device = &self.devices[idx];
        let buffer = unsafe { device.create_buffer(create_info, None)? };
        self.buffers.entry(device.handle()).and_modify(|v| v.push(buffer));
        Ok(())
    }
}
