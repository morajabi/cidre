use std::{ffi::c_void, mem::transmute};

use crate::{
    arc,
    av::{self, audio},
    blocks, cf, define_obj_type, ns, objc,
};

#[derive(Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum PrimeMethod {
    /// Primes with leading + trailing input frames.
    Pre = 0,
    /// Only primes with trailing (zero latency). Leading frames are assumed to be silence.
    Normal = 1,
    /// Acts in "latency" mode. Both leading and trailing frames assumed to be silence.
    None = 2,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct PrimeInfo {
    pub leading_frames: audio::FrameCount,
    pub trailing_frames: audio::FrameCount,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum InputStatus {
    /// This is the normal case where you supply data to the converter.
    HaveData = 0,
    /// If you are out of data for now, set *ioNumberOfPackets = 0 and return
    /// AVAudioConverterInputStatus_NoDataNow; the  conversion routine will return as much output as
    /// could be converted with the input already supplied.
    NoDataNow = 1,
    /// If you are at the end of stream, set *ioNumberOfPackets = 0 and return
    /// AVAudioConverterInputStatus_EndOfStream.
    EndOfStream = 2,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum OutputStatus {
    /// All of the requested data was returned.
    HaveData = 0,
    /// Not enough input was available to satisfy the request at the current time. The output buffer
    /// contains as much as could be converted.
    InputRanDry = 1,
    /// The end of stream has been reached. No data was returned.
    EndOfStream = 2,
    /// An error occurred.
    Error = 3,
}

define_obj_type!(pub Converter(ns::Id));

impl Converter {
    /*! @property bitRateStrategy
        @abstract When encoding, an AVEncoderBitRateStrategyKey value constant as defined in AVAudioSettings.h. Returns nil if not encoding.
    */
    // @property (nonatomic, retain, nullable) NSString *bitRateStrategy;
    #[objc::msg_send(bitRateStrategy)]
    pub fn bit_rate_strategy(&self) -> Option<&ns::String>;

    #[objc::msg_send(setBitRateStrategy:)]
    pub fn set_bit_rate_strategy(&self, value: Option<&ns::String>);

    /// The maximum size of an output packet, in bytes.
    /// When encoding it is useful to know how large a packet can be in order to allocate a buffer to receive the output.
    #[objc::msg_send(maximumOutputPacketSize)]
    pub fn maximum_output_packet_size(&self) -> isize;

    /// When encoding, an NSArray of NSNumber of all bit rates provided by the codec.
    /// Returns None if not encoding.
    #[objc::msg_send(availableEncodeBitRates)]
    pub fn available_encode_bit_rates(&self) -> Option<&ns::Array<ns::Number>>;

    /// When encoding, an cf::Array of cf::Number of bit rates that can be applied based on the current formats and settings.
    /// Returns None if not encoding.
    #[objc::msg_send(applicableEncodeBitRates)]
    pub fn applicable_encode_bit_rates(&self) -> Option<&ns::Array<ns::Number>>;

    /// When encoding, an NSArray of NSNumber of all output sample rates provided by the codec.
    /// Returns None if not encoding.
    #[objc::msg_send(availableEncodeSampleRates)]
    pub fn available_encode_sample_rates(&self) -> Option<&ns::Array<ns::Number>>;

    /// When encoding, an cf::Array of cf::Number of output sample rates that can be applied based on the current formats and settings.
    /// Returns None if not encoding.
    #[objc::msg_send(applicableEncodeSampleRates)]
    pub fn applicable_encode_sample_rates(&self) -> Option<&ns::Array<ns::Number>>;

    /// When encoding, an cf::Array of cf::Number of all output channel layout tags provided by the codec.
    /// Returns None if not encoding
    #[objc::msg_send(availableEncodeChannelLayoutTags)]
    pub fn available_encode_channel_layout_tags(&self) -> Option<&ns::Array<ns::Number>>;

    #[objc::msg_send(convertToBuffer:fromBuffer:error:)]
    pub unsafe fn convert_to_buffer_from_buffer_err<'ear>(
        &self,
        output_buffer: &mut av::AudioPcmBuf,
        from_buffer: &av::AudioPcmBuf,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    #[inline]
    pub fn convert_to_buffer_from_buffer<'ear>(
        &self,
        output_buffer: &mut av::AudioPcmBuf,
        from_buffer: &av::AudioPcmBuf,
    ) -> Result<(), &'ear ns::Error> {
        unsafe {
            let mut error = None;
            let res =
                self.convert_to_buffer_from_buffer_err(output_buffer, from_buffer, &mut error);
            if error.is_some() {
                debug_assert!(!res);
                Err(transmute(error))
            } else {
                debug_assert!(res);
                Ok(())
            }
        }
    }

    #[objc::msg_send(convertToBuffer:error:withInputFromBlock:)]
    pub unsafe fn convert_to_buffer_err_with_input_from_block(
        &self,
        output_buffer: &mut av::AudioBuf,
        error: *mut Option<&cf::Error>,
        block: *mut c_void,
    ) -> OutputStatus;

    /// Perform any supported conversion
    ///
    /// It attempts to fill the buffer to its capacity. On return, the buffer's length indicates the number of
    /// sample frames successfully converted.
    #[doc(alias = "convertToBuffer:error:withInputFromBlock:")]
    #[inline]
    pub fn convert_to_buffer_with_input_from_block<'ar, F>(
        &self,
        output_buffer: &mut av::AudioBuf,
        block: &mut blocks::Block<F>,
    ) -> Result<OutputStatus, arc::R<cf::Error>>
    where
        F: FnMut(av::audio::PacketCount, *mut InputStatus) -> Option<&'ar av::AudioBuf>,
    {
        unsafe {
            let mut error = None;
            let res = Self::convert_to_buffer_err_with_input_from_block(
                self,
                output_buffer,
                &mut error,
                block.as_mut_ptr(),
            );
            if error.is_some() {
                debug_assert_eq!(res, OutputStatus::Error);
                Err(transmute(error))
            } else {
                Ok(res)
            }
        }
    }
}
