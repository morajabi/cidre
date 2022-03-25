use crate::{define_obj_type, ns, objc::Id};

use super::{Buffer, CommandEncoder, Fence};

define_obj_type!(BlitCommandEncoder(CommandEncoder));

/// ```
/// use cidre::{mtl};
///
/// let device = mtl::Device::default().unwrap();
///
/// let command_queue = device.command_queue().unwrap();
/// let command_buffer = command_queue.command_buffer().unwrap();
///
/// let fence = device.fence().unwrap();
///
/// let mut blit_encoder = command_buffer.blit_command_encoder().unwrap();
///
/// blit_encoder.update_fence(&fence);
/// blit_encoder.end_encoding();
///
/// let mut compute_encoder = command_buffer.compute_command_encoder().unwrap();
/// compute_encoder.wait_for_fence(&fence);
/// compute_encoder.end_encoding();
///
/// command_buffer.commit();
/// command_buffer.wait_until_completed();
///
/// ```
impl BlitCommandEncoder {
    #[inline]
    pub fn update_fence(&self, fence: &Fence) {
        unsafe { wsel_updateFence(self, fence) }
    }

    #[inline]
    pub fn wait_for_fence(&self, fence: &Fence) {
        unsafe { wsel_waitForFence(self, fence) }
    }

    #[inline]
    pub fn fill_buffer(&self, buffer: &Buffer, range: ns::Range, value: u8) {
        unsafe { wsel_fillBuffer(self, buffer, range, value) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {

    fn wsel_updateFence(id: &Id, fence: &Fence);
    fn wsel_waitForFence(id: &Id, fence: &Fence);

    fn wsel_fillBuffer(id: &Id, buffer: &Buffer, range: ns::Range, value: u8);

}
