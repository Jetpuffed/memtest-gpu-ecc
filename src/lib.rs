use ash::{prelude::VkResult, vk, Device, Instance};
use std::collections::HashMap;

pub struct GpuProperties {
    physical_device_properties: vk::PhysicalDeviceProperties,
    physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
}

impl GpuProperties {
    pub fn physical_device_properties(&self) -> &vk::PhysicalDeviceProperties {
        &self.physical_device_properties
    }

    pub fn physical_device_memory_properties(&self) -> &vk::PhysicalDeviceMemoryProperties {
        &self.physical_device_memory_properties
    }
}

pub struct Gpu<'a> {
    instance: &'a Instance,
    handle: &'a vk::PhysicalDevice,
    devices: Vec<Device>,
    allocations: HashMap<vk::Device, Vec<vk::DeviceMemory>>,
    buffers: HashMap<vk::Device, Vec<vk::Buffer>>,
    properties: GpuProperties,
}

impl<'a> Gpu<'a> {
    pub fn new(instance: &'a Instance, handle: &'a vk::PhysicalDevice) -> Self {
        Self {
            instance,
            handle,
            devices: Vec::new(),
            allocations: HashMap::new(),
            buffers: HashMap::new(),
            properties: GpuProperties {
                physical_device_properties: unsafe {
                    instance.get_physical_device_properties(*handle)
                },
                physical_device_memory_properties: unsafe {
                    instance.get_physical_device_memory_properties(*handle)
                },
            },
        }
    }

    /// Returns a reference to the device at `idx`
    fn device(&self, idx: usize) -> &Device {
        &self.devices[idx]
    }

    /// Creates a new logical device and appends it to the `devices` vec
    pub fn new_device(&mut self, create_info: &vk::DeviceCreateInfo) -> VkResult<()> {
        let device = unsafe {
            self.instance
                .create_device(*self.handle, create_info, None)?
        };
        self.devices.push(device);
        let device = self.device(self.devices.len() - 1).handle();
        self.allocations.insert(device, Vec::new());
        self.buffers.insert(device, Vec::new());
        Ok(())
    }

    pub fn new_allocation(
        &mut self,
        idx: usize,
        create_info: &vk::MemoryAllocateInfo,
    ) -> VkResult<()> {
        let device = self.device(idx);
        let allocation = unsafe { device.allocate_memory(create_info, None)? };
        self.allocations
            .entry(device.handle())
            .and_modify(|v| v.push(allocation));
        Ok(())
    }

    pub fn new_buffer(&mut self, idx: usize, create_info: &vk::BufferCreateInfo) -> VkResult<()> {
        let device = self.device(idx);
        let buffer = unsafe { device.create_buffer(create_info, None)? };
        self.buffers
            .entry(device.handle())
            .and_modify(|v| v.push(buffer));
        Ok(())
    }

    pub fn bind_buffer(&mut self, idx: usize, buf_idx: usize, alc_idx: usize) -> VkResult<()> {
        let device = self.device(idx);
        let device_handle = device.handle();
        unsafe {
            device.bind_buffer_memory(
                self.buffers[&device_handle][buf_idx],
                self.allocations[&device_handle][alc_idx],
                0,
            )?
        };
        Ok(())
    }

    pub fn properties(&self) -> &GpuProperties {
        &self.properties
    }
}
