use std::{ffi::c_void, intrinsics::transmute};

use crate::{
    cf::{self, Retained},
    cm::{self, SampleBuffer, VideoCodecType},
    cv, define_cf_type, os, vt,
};

pub type OutputCallback<O, F> = extern "C" fn(
    output_ref_con: *mut O,
    source_frame_ref_con: *mut F,
    status: os::Status,
    info_flags: vt::DecodeInfoFlags,
    image_buffer: Option<&cv::ImageBuffer>,
    pts: cm::Time,
    duration: cm::Time,
);

#[repr(C)]
pub struct OutputCallbackRecord<O, F> {
    pub callback: OutputCallback<O, F>,
    pub output_ref_con: *mut O,
}

define_cf_type!(Session(vt::Session));

impl Session {
    pub fn new<O, F>(
        video_format_description: &cm::VideoFormatDescription,
        video_decoder_specification: Option<&cf::Dictionary>,
        destination_image_buffer_attirbutes: Option<&cf::Dictionary>,
        output_callback: Option<&OutputCallbackRecord<O, F>>,
    ) -> Result<cf::Retained<Self>, os::Status> {
        unsafe {
            let mut session = None;
            Self::create(
                None,
                video_format_description,
                video_decoder_specification,
                destination_image_buffer_attirbutes,
                transmute(output_callback),
                &mut session,
            )
            .to_result(session)
        }
    }

    /// Use safe new
    pub unsafe fn create(
        allocator: Option<&cf::Allocator>,
        video_format_description: &cm::VideoFormatDescription,
        video_decoder_specification: Option<&cf::Dictionary>,
        destination_image_buffer_attirbutes: Option<&cf::Dictionary>,
        output_callback: Option<&OutputCallbackRecord<c_void, c_void>>,
        decompression_session_out: &mut Option<cf::Retained<Session>>,
    ) -> os::Status {
        VTDecompressionSessionCreate(
            allocator,
            video_format_description,
            video_decoder_specification,
            destination_image_buffer_attirbutes,
            output_callback,
            decompression_session_out,
        )
    }

    /// Tears down a decompression session
    /// When you are done with a decompression session you created, call VTDecompressionSessionInvalidate
    /// to tear it down and then CFRelease to release your object reference.
    /// When a decompression session's retain count reaches zero, it is automatically invalidated, but
    /// since sessions may be retained by multiple parties, it can be hard to predict when this will happen.
    /// Calling VTDecompressionSessionInvalidate ensures a deterministic, orderly teardown.
    pub fn invalide(&mut self) {
        unsafe { VTDecompressionSessionInvalidate(self) }
    }

    pub fn decode_frame<F>(
        &self,
        sample_buffer: &SampleBuffer,
        decode_flags: vt::DecodeFrameFlags,
        source_frame_ref_con: *mut F,
        info_flags_out: *mut vt::DecodeInfoFlags,
    ) -> Result<(), os::Status> {
        unsafe {
            VTDecompressionSessionDecodeFrame(
                self,
                sample_buffer,
                decode_flags,
                transmute(source_frame_ref_con),
                info_flags_out,
            )
            .result()
        }
    }

    pub fn finish_delayed_frames(&mut self) -> Result<(), os::Status> {
        unsafe { VTDecompressionSessionFinishDelayedFrames(self).result() }
    }

    pub fn wait_for_async_frames(&mut self) -> Result<(), os::Status> {
        unsafe { VTDecompressionSessionWaitForAsynchronousFrames(self).result() }
    }

    #[inline]
    pub fn can_accept_format_description(
        &self,
        format_description: &cm::FormatDescription,
    ) -> bool {
        unsafe { VTDecompressionSessionCanAcceptFormatDescription(self, format_description) }
    }

    #[inline]
    pub fn copy_black_pixel_buffer(&self) -> Result<Retained<cv::PixelBuffer>, os::Status> {
        let mut pixel_buffer_out = None;
        unsafe {
            VTDecompressionSessionCopyBlackPixelBuffer(self, &mut pixel_buffer_out)
                .to_result(pixel_buffer_out)
        }
    }
}

pub fn is_hardware_decode_supported(codec_type: VideoCodecType) -> bool {
    unsafe { VTIsHardwareDecodeSupported(codec_type) }
}

extern "C" {

    fn VTDecompressionSessionCreate(
        allocator: Option<&cf::Allocator>,
        video_format_description: &cm::VideoFormatDescription,
        video_decoder_specification: Option<&cf::Dictionary>,
        destination_image_buffer_attirbutes: Option<&cf::Dictionary>,
        output_callback: Option<&OutputCallbackRecord<c_void, c_void>>,
        decompression_session_out: &mut Option<cf::Retained<Session>>,
    ) -> os::Status;

    fn VTDecompressionSessionInvalidate(session: &Session);

    fn VTDecompressionSessionDecodeFrame(
        session: &Session,
        sample_buffer: &SampleBuffer,
        decode_flags: vt::DecodeFrameFlags,
        source_frame_ref_con: *mut c_void,
        info_flags_out: *mut vt::DecodeInfoFlags,
    ) -> os::Status;

    fn VTDecompressionSessionFinishDelayedFrames(session: &Session) -> os::Status;
    fn VTDecompressionSessionWaitForAsynchronousFrames(session: &Session) -> os::Status;

    fn VTDecompressionSessionCanAcceptFormatDescription(
        session: &Session,
        new_fromat: &cm::FormatDescription,
    ) -> bool;

    fn VTDecompressionSessionCopyBlackPixelBuffer(
        session: &Session,
        pixel_buffer_out: *mut Option<cf::Retained<cv::PixelBuffer>>,
    ) -> os::Status;

    fn VTIsHardwareDecodeSupported(codec_type: VideoCodecType) -> bool;

}
