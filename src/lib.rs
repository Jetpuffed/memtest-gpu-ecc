use std::collections::HashMap;

use ash::{prelude::VkResult, vk, Device, Instance};

pub struct Gpu<'a> {
    instance: &'a Instance,
    handle: &'a vk::PhysicalDevice,
    device: Option<Device>,
    properties: GpuProperties,
}

impl<'a> Gpu<'a> {
    pub fn new(instance: &'a Instance, handle: &'a vk::PhysicalDevice) -> Self {
        Self {
            instance,
            handle,
            device: None,
            properties: GpuProperties::new(instance, handle),
        }
    }

    pub fn properties(&self) -> &GpuProperties {
        &self.properties
    }

    pub fn new_device(&mut self, create_info: &vk::DeviceCreateInfo) -> VkResult<()> {
        if self.device.is_some() {
            dbg!("Logical device already exists. Drop the old device first before creating a new one.");
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        let device = unsafe {
            self.instance
                .create_device(*self.handle, create_info, None)?
        };
        self.device = Some(device);
        Ok(())
    }
}

pub struct GpuResource<'a> {
    device: &'a Device,
    allocation: Option<vk::DeviceMemory>,
    buffers: Vec<vk::Buffer>,
}

impl<'a> GpuResource<'a> {
    pub fn new(device: &'a Device) -> Self {
        Self {
            device,
            allocation: None,
            buffers: Vec::new(),
        }
    }
}

pub struct GpuProperties {
    physical_device_properties: vk::PhysicalDeviceProperties,
    physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
    physical_device_features: vk::PhysicalDeviceFeatures,
    queue_family_properties: Vec<vk::QueueFamilyProperties>,
}

impl GpuProperties {
    pub fn new(instance: &Instance, physical_device: &vk::PhysicalDevice) -> Self {
        Self {
            physical_device_properties: unsafe {
                instance.get_physical_device_properties(*physical_device)
            },
            physical_device_memory_properties: unsafe {
                instance.get_physical_device_memory_properties(*physical_device)
            },
            physical_device_features: unsafe {
                instance.get_physical_device_features(*physical_device)
            },
            queue_family_properties: unsafe {
                instance.get_physical_device_queue_family_properties(*physical_device)
            },
        }
    }

    pub fn physical_device_properties(&self) -> &vk::PhysicalDeviceProperties {
        &self.physical_device_properties
    }

    pub fn physical_device_memory_properties(&self) -> &vk::PhysicalDeviceMemoryProperties {
        &self.physical_device_memory_properties
    }

    pub fn physical_device_features(&self) -> &vk::PhysicalDeviceFeatures {
        &self.physical_device_features
    }

    pub fn queue_family_properties(&self) -> &[vk::QueueFamilyProperties] {
        &self.queue_family_properties
    }
}
