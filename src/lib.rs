use ash::{prelude::VkResult, vk, Device, Instance};

pub const KILOBYTE: u64 = 1024;
pub const MEGABYTE: u64 = 1024u64.pow(2);
pub const GIGABYTE: u64 = 1024u64.pow(3);

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

    pub fn instance(&self) -> &Instance {
        &self.instance
    }

    pub fn handle(&self) -> &vk::PhysicalDevice {
        &self.handle
    }

    pub fn device(&self) -> Option<&Device> {
        if let Some(device) = &self.device {
            return Some(device)
        }
        None
    }

    pub fn properties(&self) -> &GpuProperties {
        &self.properties
    }
}

pub struct GpuProperties {
    features: vk::PhysicalDeviceFeatures,
    properties: vk::PhysicalDeviceProperties,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    queue_family_properties: Vec<vk::QueueFamilyProperties>,
}

impl GpuProperties {
    pub fn new(instance: &Instance, physical_device: &vk::PhysicalDevice) -> Self {
        Self {
            features: unsafe { instance.get_physical_device_features(*physical_device) },
            properties: unsafe { instance.get_physical_device_properties(*physical_device) },
            memory_properties: unsafe { instance.get_physical_device_memory_properties(*physical_device) },
            queue_family_properties: unsafe { instance.get_physical_device_queue_family_properties(*physical_device) },
        }
    }

    pub fn features(&self) -> &vk::PhysicalDeviceFeatures {
        &self.features
    }

    pub fn properties(&self) -> &vk::PhysicalDeviceProperties {
        &self.properties
    }

    pub fn memory_properties(&self) -> &vk::PhysicalDeviceMemoryProperties {
        &self.memory_properties
    }

    pub fn queue_family_properties(&self) -> &[vk::QueueFamilyProperties] {
        &self.queue_family_properties
    }

    pub fn find_queue_families(&self, flags: vk::QueueFlags) -> Option<Vec<usize>> {
        let indices = self.queue_family_properties()
            .iter()
            .enumerate()
            .filter(|(_, v)| (v.queue_flags.as_raw() & flags.as_raw()) != 0)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        if indices.len() == 0 {
            return None
        }
        Some(indices)
    }
}
