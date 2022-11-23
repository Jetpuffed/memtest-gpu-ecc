use super::*;

fn write_pattern(dev: &Device, cmd_buf: &vk::CommandBuffer, buf: &vk::Buffer, pat: u32) {
    unsafe { dev.cmd_fill_buffer(*cmd_buf, *buf, 0, vk::WHOLE_SIZE, pat) }
}
