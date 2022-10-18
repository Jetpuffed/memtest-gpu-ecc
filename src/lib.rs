use ash::{prelude::VkResult, vk, Device, Instance};

pub const KILOBYTE: u64 = 1024;
pub const MEGABYTE: u64 = 1024u64.pow(2);
pub const GIGABYTE: u64 = 1024u64.pow(3);

pub struct Gpu<'a> {
    instance: &'a Instance,
    handle: &'a vk::PhysicalDevice,
    device: Option<Device>,
    memory: Option<vk::DeviceMemory>,
    properties: GpuProperties,
}

impl<'a> Gpu<'a> {
    pub fn new(instance: &'a Instance, handle: &'a vk::PhysicalDevice) -> Self {
        Self {
            instance,
            handle,
            device: None,
            memory: None,
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
            return Some(device);
        }
        None
    }

    pub fn memory(&self) -> Option<&vk::DeviceMemory> {
        if let Some(memory) = &self.memory {
            return Some(memory);
        }
        None
    }

    pub fn properties(&self) -> &GpuProperties {
        &self.properties
    }

    pub fn new_device(&mut self, queues: i32, flags: vk::QueueFlags) {
        let queue_family_indices = self
            .properties
            .find_queue_families(flags)
            .expect("queue family not found");
        let queue_priorities = vec![1.0; queues as usize];
        let queue_create_infos = [vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(queue_family_indices[0] as u32) // first index will always be suitable
            .queue_priorities(&queue_priorities)
            .build()];
        let enabled_layer_names = self
            .properties
            .layers
            .iter()
            .map(|p| p.layer_name.as_ptr())
            .collect::<Vec<*const i8>>();
        let enabled_extension_names = self
            .properties
            .extensions
            .iter()
            .map(|p| p.extension_name.as_ptr())
            .collect::<Vec<*const i8>>();
        let enabled_features = &self.properties.features;
        let create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_create_infos)
            .enabled_layer_names(&enabled_layer_names) // deprecated, but adding for compatibility
            .enabled_extension_names(&enabled_extension_names)
            .enabled_features(enabled_features);
        self.device = unsafe {
            self.instance
                .create_device(*self.handle, &create_info, None)
                .ok()
        };
    }

    pub fn allocate_memory(&mut self, size: u64) {
        let device = self.device.as_ref().expect("device not found");
        let memory_type_index = self
            .properties
            .find_memory_type(vk::MemoryPropertyFlags::DEVICE_LOCAL)
            .expect("memory type not found"); // should use device memory, not host memory
        let create_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(size)
            .memory_type_index(memory_type_index as u32);
        self.memory = unsafe { device.allocate_memory(&create_info, None).ok() };
    }
}

pub struct GpuProperties {
    layers: Vec<vk::LayerProperties>,
    extensions: Vec<vk::ExtensionProperties>,
    features: vk::PhysicalDeviceFeatures,
    properties: vk::PhysicalDeviceProperties,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    queue_family_properties: Vec<vk::QueueFamilyProperties>,
}

impl GpuProperties {
    pub fn new(instance: &Instance, physical_device: &vk::PhysicalDevice) -> Self {
        Self {
            layers: unsafe {
                instance
                    .enumerate_device_layer_properties(*physical_device)
                    .unwrap()
            },
            extensions: unsafe {
                instance
                    .enumerate_device_extension_properties(*physical_device)
                    .unwrap()
            },
            features: unsafe { instance.get_physical_device_features(*physical_device) },
            properties: unsafe { instance.get_physical_device_properties(*physical_device) },
            memory_properties: unsafe {
                instance.get_physical_device_memory_properties(*physical_device)
            },
            queue_family_properties: unsafe {
                instance.get_physical_device_queue_family_properties(*physical_device)
            },
        }
    }

    pub fn layers(&self) -> &[vk::LayerProperties] {
        &self.layers
    }

    pub fn extensions(&self) -> &[vk::ExtensionProperties] {
        &self.extensions
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
        let indices = self
            .queue_family_properties()
            .iter()
            .enumerate()
            .filter(|(_, v)| (v.queue_flags.as_raw() & flags.as_raw()) != 0)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        if indices.len() == 0 {
            return None;
        }
        Some(indices)
    }

    pub fn find_memory_type(&self, flags: vk::MemoryPropertyFlags) -> Option<usize> {
        let index = self
            .memory_properties
            .memory_types
            .iter()
            .position(|m| (m.property_flags.as_raw() & flags.as_raw()) != 0);
        index
    }
}
