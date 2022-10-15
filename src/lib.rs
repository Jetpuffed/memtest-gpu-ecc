use ash::{prelude::VkResult, vk, Device, Instance};

pub const KILOBYTE: u64 = 1024;
pub const MEGABYTE: u64 = 1024u64.pow(2);
pub const GIGABYTE: u64 = 1024u64.pow(3);

pub struct Gpu<'a, 'b> {
    instance: &'a Instance,
    handle: &'a vk::PhysicalDevice,
    device: Option<Device>,
    resources: Vec<GpuResource<'b>>,
    properties: GpuProperties,
}

impl<'a, 'b> Gpu<'a, 'b> {
    pub fn new(instance: &'a Instance, handle: &'a vk::PhysicalDevice) -> Self {
        Self {
            instance,
            handle,
            device: None,
            resources: Vec::new(),
            properties: GpuProperties::new(instance, handle),
        }
    }

    pub fn device(&self) -> Option<&Device> {
        if let Some(device) = &self.device {
            return Some(device)
        }
        None
    }

    pub fn resources(&self, idx: usize) -> Option<&GpuResource> {
        self.resources.get(idx)
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

    pub fn new_resource(&'b mut self) -> VkResult<()> {
        if let Some(device) = &self.device {
            let resource = GpuResource::new(device);
            self.resources.push(resource);
            return Ok(())
        }
        dbg!("Device not found. Create a new logical device first before attemping to create a new resource.");
        Err(vk::Result::ERROR_INITIALIZATION_FAILED)
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

    pub fn allocation(&self) -> vk::DeviceMemory {
        if self.allocation.is_none() {
            return vk::DeviceMemory::null();
        }
        self.allocation.unwrap()
    }

    fn new_buffer(&mut self, create_info: &vk::BufferCreateInfo) -> VkResult<()> {
        let buffer = unsafe { self.device.create_buffer(create_info, None)? };
        self.buffers.push(buffer);
        Ok(())
    }

    pub fn new_src_buffer(&mut self, size: u64) -> VkResult<()> {
        let buffer_info = vk::BufferCreateInfo::builder()
            .size(size)
            .usage(vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_SRC);
        self.new_buffer(&buffer_info)
    }

    pub fn new_dst_buffer(&mut self, size: u64) -> VkResult<()> {
        let buffer_info = vk::BufferCreateInfo::builder()
            .size(size)
            .usage(vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST);
        self.new_buffer(&buffer_info)
    }

    pub fn new_allocation(&mut self, create_info: &vk::MemoryAllocateInfo) -> VkResult<()> {
        if self.allocation.is_some() {
            dbg!("An allocation already exists. Drop the old allocation first before creating a new one.");
            return Err(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY);
        }
        let allocation = unsafe { self.device.allocate_memory(create_info, None)? };
        self.allocation = Some(allocation);
        Ok(())
    }

    pub fn bind_memory(&self, bind_infos: &[vk::BindBufferMemoryInfo]) -> VkResult<()> {
        unsafe { self.device.bind_buffer_memory2(bind_infos) }
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
