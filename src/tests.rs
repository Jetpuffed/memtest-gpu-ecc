use super::*;

fn write_zeros(dev: &Device, cmd_buf: &vk::CommandBuffer, buf: &vk::Buffer) {
    unsafe { dev.cmd_fill_buffer(*cmd_buf, *buf, 0, vk::WHOLE_SIZE, 0x0) }
}

fn write_ones(dev: &Device, cmd_buf: &vk::CommandBuffer, buf: &vk::Buffer) {
    unsafe { dev.cmd_fill_buffer(*cmd_buf, *buf, 0, vk::WHOLE_SIZE, 0xFFFFFFFF) }
}
