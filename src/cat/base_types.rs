// CF_ENUM(OSStatus)
// {
//     kAudio_UnimplementedError     = -4,
//     kAudio_FileNotFoundError      = -43,
//     kAudio_FilePermissionError    = -54,
//     kAudio_TooManyFilesOpenError  = -42,
//     kAudio_BadFilePathError       = '!pth', // 0x21707468, 561017960
//     kAudio_ParamError             = -50,
//     kAudio_MemFullError           = -108
// };

use std::ffi::c_void;

use crate::os;

/// These are the error codes returned from the APIs found through Core Audio related frameworks.
pub mod os_status {
    use crate::os::Status;

    pub const UNIMPLEMENTED_ERROR: Status = Status(-4);
    pub const FILE_NOT_FOUND_ERROR: Status = Status(-43);
    pub const FILE_PERMISSION_ERROR: Status = Status(-54);
    pub const TOO_MANY_FILES_OPEN_ERROR: Status = Status(-42);
    pub const BAD_FILE_PATH_ERROR: Status = Status(i32::from_be_bytes(*b"!pth"));
    pub const PARAM_ERROR: Status = Status(-50);
    pub const MEM_FULL_ERROR: Status = Status(-108);
}

/// This structure holds a pair of numbers that represent a continuous range of values.
#[repr(C)]
pub struct AudioValueRange {
    pub minimum: f64,
    pub maximum: f64,
}

/// A structure to hold a buffer of audio data.
#[repr(C)]
pub struct AudioBuffer {
    /// The number of interleaved channels in the buffer.
    pub number_channels: u32,
    /// The number of bytes in the buffer pointed at by mData.
    pub data_bytes_syte: u32,
    /// A pointer to the buffer of audio data.
    pub data: *mut c_void,
}

#[repr(C)]
pub struct AudioBufferList {
    pub number_buffers: u32,
    /// this is a variable length array of mNumberBuffers elements
    pub buffers: *mut AudioBuffer,
}

/// A four char code indicating the general kind of data in the stream.
#[repr(transparent)]
pub struct AudioFormatID(pub u32);

/// The AudioFormatIDs used to identify individual formats of audio data.
impl AudioFormatID {
    /// Linear PCM, uses the standard flags.
    pub const LINEAR_PC: Self = Self(u32::from_be_bytes(*b"lpcm"));

    /// AC-3, has no flags.
    pub const AC3: Self = Self(u32::from_be_bytes(*b"ac-3"));

    /// AC-3 packaged for transport over an IEC 60958 compliant digital audio
    /// interface. Uses the standard flags.
    pub const _60958AC3: Self = Self(u32::from_be_bytes(*b"cac3"));

    /// Apples implementation of IMA 4:1 ADPCM, has no flags.
    pub const APPLE_IMA4: Self = Self(u32::from_be_bytes(*b"ima4"));

    /// MPEG-4 Low Complexity AAC audio object, has no flags.
    pub const MPEG4_AAC: Self = Self(u32::from_be_bytes(*b"aac "));

    /// MPEG-4 CELP audio object, has no flags.
    pub const MPEG4_CELP: Self = Self(u32::from_be_bytes(*b"celp"));

    /// MPEG-4 HVXC audio object, has no flags.
    pub const MPEG4_HVXC: Self = Self(u32::from_be_bytes(*b"hvxc"));

    /// MPEG-4 TwinVQ audio object type, has no flags.
    pub const MPEG4_TWIN_VQ: Self = Self(u32::from_be_bytes(*b"twvq"));

    /// MACE 3:1, has no flags.
    pub const MACE3: Self = Self(u32::from_be_bytes(*b"MAC3"));

    /// MACE 6:1, has no flags.
    pub const MACE6: Self = Self(u32::from_be_bytes(*b"MAC6"));

    /// µLaw 2:1, has no flags.
    pub const U_LAW: Self = Self(u32::from_be_bytes(*b"ulaw"));

    /// aLaw 2:1, has no flags.
    pub const A_LAW: Self = Self(u32::from_be_bytes(*b"alaw"));

    /// QDesign music, has no flags
    pub const Q_DESIGN: Self = Self(u32::from_be_bytes(*b"QDMC"));

    /// QDesign2 music, has no flags
    pub const Q_DESIGN2: Self = Self(u32::from_be_bytes(*b"QDM2"));

    /// QUALCOMM PureVoice, has no flags
    pub const QUALCOMM: Self = Self(u32::from_be_bytes(*b"Qclp"));

    /// MPEG-1/2, Layer 1 audio, has no flags
    pub const MPEGLAYER1: Self = Self(u32::from_be_bytes(*b".mp1"));

    /// MPEG-1/2, Layer 2 audio, has no flags
    pub const MPEGLAYER2: Self = Self(u32::from_be_bytes(*b".mp2"));

    /// MPEG-1/2, Layer 3 audio, has no flags
    pub const MPEGLAYER3: Self = Self(u32::from_be_bytes(*b".mp3"));

    /// A stream of IOAudioTimeStamps, uses the IOAudioTimeStamp flags (see
    /// IOKit/audio/IOAudioTypes.h).
    pub const TIME_CODE: Self = Self(u32::from_be_bytes(*b"time"));

    /// A stream of MIDIPacketLists where the time stamps in the MIDIPacketList are
    /// sample offsets in the stream. The mSampleRate field is used to describe how
    /// time is passed in this kind of stream and an AudioUnit that receives or
    /// generates this stream can use this sample rate, the number of frames it is
    /// rendering and the sample offsets within the MIDIPacketList to define the
    /// time for any MIDI event within this list. It has no flags.
    pub const MIDI_STREAM: Self = Self(u32::from_be_bytes(*b"midi"));

    /// A "side-chain" of Float32 data that can be fed or generated by an AudioUnit
    /// and is used to send a high density of parameter value control information.
    /// An AU will typically run a ParameterValueStream at either the sample rate of
    /// the AudioUnit's audio data, or some integer divisor of this (say a half or a
    /// third of the sample rate of the audio). The Sample Rate of the ASBD
    /// describes this relationship. It has no flags.
    pub const PARAMETER_VALUE_STREAM: Self = Self(u32::from_be_bytes(*b"apvs"));

    /// Apple Lossless, the flags indicate the bit depth of the source material.
    pub const APPLE_LOSSLESS: Self = Self(u32::from_be_bytes(*b"alac"));

    /// MPEG-4 High Efficiency AAC audio object, has no flags.
    pub const MPEG4_AAC_HE: Self = Self(u32::from_be_bytes(*b"aach"));

    /// MPEG-4 AAC Low Delay audio object, has no flags.
    pub const MPEG4_AAC_LD: Self = Self(u32::from_be_bytes(*b"aacl"));

    /// MPEG-4 AAC Enhanced Low Delay audio object, has no flags. This is the formatID of
    /// the base layer without the SBR extension. See also kAudioFormatMPEG4AAC_ELD_SBR
    pub const MPEG4_AAC_ELD: Self = Self(u32::from_be_bytes(*b"aace"));

    /// MPEG-4 AAC Enhanced Low Delay audio object with SBR extension layer, has no flags.
    pub const MPEG4_AAC_ELD_SBR: Self = Self(u32::from_be_bytes(*b"aacf"));

    pub const MPEG4_AAC_ELD_V2: Self = Self(u32::from_be_bytes(*b"aacg"));

    /// MPEG-4 High Efficiency AAC Version 2 audio object, has no flags.    
    pub const MPEG4_AAC_HE_V2: Self = Self(u32::from_be_bytes(*b"aacp"));

    /// MPEG-4 Spatial Audio audio object, has no flags.
    pub const MPEG4_AAC_SPATIAL: Self = Self(u32::from_be_bytes(*b"aacs"));

    /// MPEG-D Unified Speech and Audio Coding, has no flags.
    pub const MPEG_D_USAC: Self = Self(u32::from_be_bytes(*b"usac"));

    /// The AMR Narrow Band speech codec.
    pub const AMR: Self = Self(u32::from_be_bytes(*b"samr"));

    /// The AMR Wide Band speech codec.
    pub const AMR_WB: Self = Self(u32::from_be_bytes(*b"sawb"));

    /// The format used for Audible audio books. It has no flags.
    pub const AUDIBLE: Self = Self(u32::from_be_bytes(*b"AUDB"));

    /// The iLBC narrow band speech codec. It has no flags.
    pub const LBC: Self = Self(u32::from_be_bytes(*b"ilbc"));

    /// DVI/Intel IMA ADPCM - ACM code 17.
    pub const DVI_INTEL_IMA: Self = Self(0x6D730011);

    /// Microsoft GSM 6.10 - ACM code 49.
    pub const MICROSOFT_GSM: Self = Self(0x6D730031);

    /// This format is defined by AES3-2003, and adopted into MXF and MPEG-2
    /// containers and SDTI transport streams with SMPTE specs 302M-2002 and
    /// 331M-2000. It has no flags.
    pub const AES3: Self = Self(u32::from_be_bytes(*b"aes3"));

    /// Enhanced AC-3, has no flags.
    pub const ENHANCED_AC3: Self = Self(u32::from_be_bytes(*b"ec-3"));

    /// Free Lossless Audio Codec, the flags indicate the bit depth of the source material.
    pub const FLAC: Self = Self(u32::from_be_bytes(*b"flac"));

    /// Opus codec, has no flags.
    pub const OPUS: Self = Self(u32::from_be_bytes(*b"opus"));
}

/// Flags that are specific to each format.
#[repr(transparent)]
pub struct AudioFormatFlags(pub u32);

/// These are the standard AudioFormatFlags for use in the mFormatFlags field of the
/// AudioStreamBasicDescription structure.
/// Typically, when an ASBD is being used, the fields describe the complete layout
/// of the sample data in the buffers that are represented by this description -
/// where typically those buffers are represented by an AudioBuffer that is
/// contained in an AudioBufferList.
///
/// However, when an ASBD has the kAudioFormatFlagIsNonInterleaved flag, the
/// AudioBufferList has a different structure and semantic. In this case, the ASBD
/// fields will describe the format of ONE of the AudioBuffers that are contained in
/// the list, AND each AudioBuffer in the list is determined to have a single (mono)
/// channel of audio data. Then, the ASBD's mChannelsPerFrame will indicate the
/// total number of AudioBuffers that are contained within the AudioBufferList -
/// where each buffer contains one channel. This is used primarily with the
/// AudioUnit (and AudioConverter) representation of this list - and won't be found
/// in the AudioHardware usage of this structure.
impl AudioFormatFlags {
    /// Set for floating point, clear for integer.
    pub const IS_FLOAT: Self = Self(1u32 << 0);

    /// Set for big endian, clear for little endian.
    pub const IS_BIG_ENDIAN: Self = Self(1u32 << 1);

    /// Set for signed integer, clear for unsigned integer. This is only valid if
    /// kAudioFormatFlagIsFloat is clear.
    pub const IS_SIGNED_INTEGER: Self = Self(1u32 << 2);

    /// Set if the sample bits occupy the entire available bits for the channel,
    /// clear if they are high or low aligned within the channel. Note that even if
    /// this flag is clear, it is implied that this flag is set if the
    /// AudioStreamBasicDescription is filled out such that the fields have the
    /// following relationship:
    ///     ((mBitsPerSample / 8) * mChannelsPerFrame) == mBytesPerFrame
    pub const IS_PACKED: Self = Self(1u32 << 3);

    /// Set if the sample bits are placed into the high bits of the channel, clear
    /// for low bit placement. This is only valid if kAudioFormatFlagIsPacked is
    /// clear.
    pub const IS_ALIGNED_HIGH: Self = Self(1u32 << 4);

    /// Set if the samples for each channel are located contiguously and the
    /// channels are layed out end to end, clear if the samples for each frame are
    /// layed out contiguously and the frames layed out end to end.
    pub const IS_NON_INTERLEAVED: Self = Self(1u32 << 5);

    /// Set to indicate when a format is non-mixable. Note that this flag is only
    /// used when interacting with the HAL's stream format information. It is not a
    /// valid flag for any other uses.
    pub const IS_NON_MIXABLE: Self = Self(1u32 << 6);

    /// Set if all the flags would be clear in order to preserve 0 as the wild card
    /// value.
    pub const ARE_ALL_CLEAR: Self = Self(0x80000000);

    pub const LINEAR_PCM_IS_FLOAT: Self = Self::IS_FLOAT;
    pub const LINEAR_PCM_IS_BIG_ENDIAN: Self = Self::IS_BIG_ENDIAN;
    pub const LINEAR_PCM_IS_SIGNED_INTEGER: Self = Self::IS_SIGNED_INTEGER;
    pub const LINEAR_PCM_IS_PACKED: Self = Self::IS_PACKED;
    pub const LINEAR_PCM_IS_ALIGNED_HIGH: Self = Self::IS_ALIGNED_HIGH;
    pub const LINEAR_PCM_IS_NON_INTERLEAVED: Self = Self::IS_NON_INTERLEAVED;
    pub const LINEAR_PCM_IS_NON_MIXABLE: Self = Self::IS_NON_MIXABLE;

    /// The linear PCM flags contain a 6-bit bitfield indicating that an integer
    /// format is to be interpreted as fixed point. The value indicates the number
    /// of bits are used to represent the fractional portion of each sample value.
    /// This constant indicates the bit position (counting from the right) of the
    /// bitfield in mFormatFlags.
    pub const LINEAR_PCM_SAMPLE_FRACTION_SHIFT: Self = Self(7);

    /// number_fractional_bits = (mFormatFlags &
    /// kLinearPCMFormatFlagsSampleFractionMask) >>
    /// kLinearPCMFormatFlagsSampleFractionShift
    pub const LINEAR_PCM_SAMPLE_FRACTION_MASK: Self =
        Self(0x3f << Self::LINEAR_PCM_SAMPLE_FRACTION_SHIFT.0);

    pub const LINEAR_PCM_ARE_ALL_CLEAR: Self = Self::ARE_ALL_CLEAR;

    /// This flag is set for Apple Lossless data that was sourced from 16 bit native
    /// endian signed integer data.
    pub const APPLE_LOSSLESS_16_BIT_SOURCE_DATA: Self = Self(1);

    /// This flag is set for Apple Lossless data that was sourced from 20 bit native
    /// endian signed integer data aligned high in 24 bits.
    pub const APPLE_LOSSLESS_20_BIT_SOURCE_DATA: Self = Self(2);

    /// his flag is set for Apple Lossless data that was sourced from 24 bit native
    /// endian signed integer data.
    pub const APPLE_LOSSLESS_24_BIT_SOURCE_DATA: Self = Self(3);

    /// This flag is set for Apple Lossless data that was sourced from 32 bit native
    /// endian signed integer data.
    pub const APPLE_LOSSLESS_32_BIT_SOURCE_DATA: Self = Self(4);
}

/// This structure encapsulates all the information for describing the basic
/// format properties of a stream of audio data.
///
/// This structure is sufficient to describe any constant bit rate format that  has
/// channels that are the same size. Extensions are required for variable bit rate
/// data and for constant bit rate data where the channels have unequal sizes.
/// However, where applicable, the appropriate fields will be filled out correctly
/// for these kinds of formats (the extra data is provided via separate properties).
/// In all fields, a value of 0 indicates that the field is either unknown, not
/// applicable or otherwise is inapproprate for the format and should be ignored.
/// Note that 0 is still a valid value for most formats in the mFormatFlags field.
///
/// In audio data a frame is one sample across all channels. In non-interleaved
/// audio, the per frame fields identify one channel. In interleaved audio, the per
/// frame fields identify the set of n channels. In uncompressed audio, a Packet is
/// one frame, (mFramesPerPacket == 1). In compressed audio, a Packet is an
/// indivisible chunk of compressed data, for example an AAC packet will contain
/// 1024 sample frames.
#[repr(C)]
pub struct AudioStreamBasicDescription {
    /// The number of sample frames per second of the data in the stream.
    pub sample_rate: f64,
    /// The AudioFormatID indicating the general kind of data in the stream.
    pub format_id: AudioFormatID,
    /// The AudioFormatFlags for the format indicated by mFormatID.
    pub format_flags: AudioFormatFlags,
    /// The number of bytes in a packet of data.
    pub bytes_per_packet: u32,
    /// The number of sample frames in each packet of data.
    pub frames_per_packet: u32,
    /// The number of bytes in a single sample frame of data.
    pub bytes_per_frame: u32,
    /// The number of channels in each frame of data.
    pub channels_per_frame: u32,
    /// The number of bits of sample data for each channel in a frame of data.
    pub bits_per_channel: u32,
    /// Pads the structure out to force an even 8 byte alignment.
    pub reserved: u32,
}

/// The format can use any sample rate. Note that this constant can only appear
/// in listings of supported formats. It will never appear in a current format.
pub const AUDIO_STREAM_ANY_RATE: f64 = 0.0;

#[derive(Debug, Copy, Clone, Default)]
#[repr(transparent)]
pub struct SMPTETimeType(pub u32);

impl SMPTETimeType {
    /// 24 Frame
    pub const _24: Self = Self(0);

    /// 25 Frame
    pub const _25: Self = Self(1);

    /// 30 Drop Frame
    pub const _30_DROP: Self = Self(2);

    /// 30 Frame
    pub const _30: Self = Self(3);

    /// 29.97 Frame
    pub const _29_97: Self = Self(4);

    /// 29.97 Drop Frame
    pub const _29_97_DROP: Self = Self(5);

    /// 60 Frame
    pub const _60: Self = Self(6);

    /// 59.94 Frame
    pub const _59_94: Self = Self(7);

    /// 60 Drop Frame
    pub const _60_DROP: Self = Self(8);

    /// 59.94 Drop Frame
    pub const _59_94_DROP: Self = Self(9);

    /// 50 Frame
    pub const _50: Self = Self(10);

    /// 23.98 Frame
    pub const _23_98: Self = Self(11);
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(transparent)]
pub struct SMPTETimeFlags(pub u32);

impl SMPTETimeFlags {
    pub const UNKNOWN: Self = Self(0);
    pub const VALID: Self = Self(1u32 << 0);
    pub const RUNNING: Self = Self(1u32 << 1);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct SMPTETime {
    pub subframes: i16,
    pub subframes_divisor: i16,
    pub counter: u32,
    pub r#type: SMPTETimeType,
    pub flags: SMPTETimeFlags,
    pub hours: i16,
    pub minutes: i16,
    pub seconds: i16,
    pub frames: i16,
}

#[repr(C)]
pub struct AudioTimeStamp {
    /// The absolute sample frame time.
    pub sample_time: f64,
    /// The host machine's time base, mach_absolute_time.
    pub host_time: u64,
    /// The ratio of actual host ticks per sample frame to the nominal host ticks
    /// per sample frame.
    pub rate_scalar: f64,
    /// The word clock time.
    pub work_clock_time: u64,
    /// The SMPTE time.
    pub smpte_time: SMPTETime,
    /// A set of flags indicating which representations of the time are valid.
    pub flags: AudioTimeStampFlags,
    /// Pads the structure out to force an even 8 byte alignment.
    pub reserved: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AudioTimeStampFlags(pub u32);

impl AudioTimeStampFlags {
    pub const NOTHING_VALID: Self = Self(0);
    /// The sample frame time is valid.
    pub const SAMPLE_TIME_VALID: Self = Self(1u32 << 0);
    /// The host time is valid.
    pub const HOST_TIME_VALID: Self = Self(1u32 << 1);
    /// The rate scalar is valid.
    pub const RATE_SCALAR_VALID: Self = Self(1u32 << 2);
    /// The word clock time is valid.
    pub const WORD_CLOCK_TIME_VALID: Self = Self(1u32 << 3);
    /// The SMPTE time is valid.
    pub const SMPTETIME_VALID: Self = Self(1u32 << 4);
    /// The sample frame time and the host time are valid.
    pub const SAMPLE_HOST_TIME_VALID: Self =
        Self(Self::SAMPLE_TIME_VALID.0 | Self::HOST_TIME_VALID.0);
}

#[repr(C)]
pub struct AudioClassDescription {
    /// The four char code codec type.
    pub m_type: os::Type,
    /// The four char code codec subtype.
    pub m_sub_type: os::Type,
    /// The four char code codec manufacturer.
    pub m_manufacturer: os::Type,
}

/// A tag identifying how the channel is to be used.
#[repr(transparent)]
pub struct AudioChannelLabel(pub u32);

impl AudioChannelLabel {
    /// unknown or unspecified other use
    pub const UNKNOWN: Self = Self(0xFFFFFFFF);
    /// channel is present, but has no intended use or destination
    pub const UNUSED: Self = Self(0);
    /// channel is described by the mCoordinates fields.
    pub const USE_COORDINATES: Self = Self(100);

    pub const LEFT: Self = Self(1);
    pub const RIGHT: Self = Self(2);
    pub const CENTER: Self = Self(3);
    pub const LFE_SCREEN: Self = Self(4);
    pub const LEFT_SURROUND: Self = Self(5);
    pub const RIGHT_SURROUND: Self = Self(6);
    pub const LEFT_CENTER: Self = Self(7);
    pub const RIGHT_CENTER: Self = Self(8);

    /// WAVE: "Back Center" or plain "Rear Surround"
    pub const CENTER_SURROUND: Self = Self(9);
    pub const LEFT_SURROUND_DIRECT: Self = Self(10);
    pub const RIGHT_SURROUND_DIRECT: Self = Self(11);
    pub const TOP_CENTER_SURROUND: Self = Self(12);

    /// WAVE: "Top Front Left
    pub const VERTICAL_HEIGHT_LEFT: Self = Self(13);

    /// WAVE: "Top Front Center"
    pub const VERTICAL_HEIGHT_CENTER: Self = Self(14);

    /// WAVE: "Top Front Right"
    pub const VERTICAL_HEIGHT_RIGHT: Self = Self(15);

    pub const TOP_BACK_LEFT: Self = Self(16);
    pub const TOP_BACK_CENTER: Self = Self(17);
    pub const TOP_BACK_RIGHT: Self = Self(18);

    pub const REAR_SURROUND_LEFT: Self = Self(33);
    pub const REAR_SURROUND_RIGHT: Self = Self(34);
    pub const LEFT_WIDE: Self = Self(35);
    pub const RIGHT_WIDE: Self = Self(36);
    pub const LFE2: Self = Self(37);
    /// matrix encoded 4 channels
    pub const LEFT_TOTAL: Self = Self(38);
    /// matrix encoded 4 channels
    pub const RIGHT_TOTAL: Self = Self(39);
    pub const HEARING_IMPAIRED: Self = Self(40);
    pub const NARRATION: Self = Self(41);
    pub const MONO: Self = Self(42);
    pub const DIALOG_CENTRIC_MIX: Self = Self(43);

    /// back center, non diffuse
    pub const CENTER_SURROUND_DIRECT: Self = Self(44);

    pub const HAPTIC: Self = Self(45);

    pub const LEFT_TOP_FRONT: Self = Self::VERTICAL_HEIGHT_LEFT;
    pub const CENTER_TOP_FRONT: Self = Self::VERTICAL_HEIGHT_CENTER;
    pub const RIGHT_TOP_FRONT: Self = Self::VERTICAL_HEIGHT_RIGHT;
    pub const LEFT_TOP_MIDDLE: Self = Self(49);
    pub const CENTER_TOP_MIDDLE: Self = Self::TOP_CENTER_SURROUND;
    pub const RIGHT_TOP_MIDDLE: Self = Self(51);
    pub const LEFT_TOP_REAR: Self = Self(52);
    pub const CENTER_TOP_REAR: Self = Self(53);
    pub const RIGHT_TOP_REAR: Self = Self(54);

    // first order ambisonic channels

    pub const AMBISONIC_W: Self = Self(200);
    pub const AMBISONIC_X: Self = Self(201);
    pub const AMBISONIC_Y: Self = Self(202);
    pub const AMBISONIC_Z: Self = Self(203);

    // Mid/Side Recording

    pub const MS_MID: Self = Self(204);
    pub const MS_SIDE: Self = Self(205);

    // X-Y Recording

    pub const XY_X: Self = Self(206);
    pub const XY_Y: Self = Self(207);

    // Binaural Recording
    pub const BINAURAL_LEFT: Self = Self(208);
    pub const BINAURAL_RIGHT: Self = Self(209);

    // other
    pub const HEADPHONES_LEFT: Self = Self(301);
    pub const HEADPHONES_RIGHT: Self = Self(302);
    pub const CLICK_TRACK: Self = Self(304);
    pub const FOREIGN_LANGUAGE: Self = Self(305);

    // generic discrete channel
    pub const DISCRETE: Self = Self(400);

    // numbered discrete channel
    pub const DISCRETE_0: Self = Self((1u32 << 16) | 0);
    pub const DISCRETE_1: Self = Self((1u32 << 16) | 1);
    pub const DISCRETE_2: Self = Self((1u32 << 16) | 2);
    pub const DISCRETE_3: Self = Self((1u32 << 16) | 3);
    pub const DISCRETE_4: Self = Self((1u32 << 16) | 4);
    pub const DISCRETE_5: Self = Self((1u32 << 16) | 5);
    pub const DISCRETE_6: Self = Self((1u32 << 16) | 6);
    pub const DISCRETE_7: Self = Self((1u32 << 16) | 7);
    pub const DISCRETE_8: Self = Self((1u32 << 16) | 8);
    pub const DISCRETE_9: Self = Self((1u32 << 16) | 9);
    pub const DISCRETE_10: Self = Self((1u32 << 16) | 10);
    pub const DISCRETE_11: Self = Self((1u32 << 16) | 11);
    pub const DISCRETE_12: Self = Self((1u32 << 16) | 12);
    pub const DISCRETE_13: Self = Self((1u32 << 16) | 13);
    pub const DISCRETE_14: Self = Self((1u32 << 16) | 14);
    pub const DISCRETE_15: Self = Self((1u32 << 16) | 15);
    pub const DISCRETE_65535: Self = Self((1u32 << 16) | 65535);

    // generic HOA ACN channel
    pub const HOA_ACN: Self = Self(500);

    // numbered HOA ACN channels, SN3D normalization
    pub const HOA_ACN_0: Self = Self((2u32 << 16) | 0);
    pub const HOA_ACN_1: Self = Self((2u32 << 16) | 1);
    pub const HOA_ACN_2: Self = Self((2u32 << 16) | 2);
    pub const HOA_ACN_3: Self = Self((2u32 << 16) | 3);
    pub const HOA_ACN_4: Self = Self((2u32 << 16) | 4);
    pub const HOA_ACN_5: Self = Self((2u32 << 16) | 5);
    pub const HOA_ACN_6: Self = Self((2u32 << 16) | 6);
    pub const HOA_ACN_7: Self = Self((2u32 << 16) | 7);
    pub const HOA_ACN_8: Self = Self((2u32 << 16) | 8);
    pub const HOA_ACN_9: Self = Self((2u32 << 16) | 9);
    pub const HOA_ACN_10: Self = Self((2u32 << 16) | 10);
    pub const HOA_ACN_11: Self = Self((2u32 << 16) | 11);
    pub const HOA_ACN_12: Self = Self((2u32 << 16) | 12);
    pub const HOA_ACN_13: Self = Self((2u32 << 16) | 13);
    pub const HOA_ACN_14: Self = Self((2u32 << 16) | 14);
    pub const HOA_ACN_15: Self = Self((2u32 << 16) | 15);
    pub const HOA_ACN_65024: Self = Self((2u32 << 16) | 65024); // 254th order uses 65025 channels

    // Specific SN3D alias
    pub const HOA_SN3D: Self = Self::HOA_ACN_0; // Needs to be ORed with the channel index, not HOA order

    // HOA N3D
    pub const HOA_N3D: Self = Self(3u32 << 16); // Needs to be ORed with the channel index, not HOA order

    // Channel label values in this range are reserved for internal use
    pub const BEGIN_RESERVED: Self = Self(0xF0000000);
    pub const END_RESERVED: Self = Self(0xFFFFFFFE);
}

#[repr(transparent)]
pub struct AudioChannelBitmap(pub u32);

impl AudioChannelBitmap {
    pub const LEFT: Self = Self(1u32 << 0);
    pub const RIGHT: Self = Self(1u32 << 1);
    pub const CENTER: Self = Self(1u32 << 2);
    pub const LFE_SCREEN: Self = Self(1u32 << 3);
    pub const LEFT_SURROUND: Self = Self(1u32 << 4);
    pub const RIGHT_SURROUND: Self = Self(1u32 << 5);
    pub const LEFT_CENTER: Self = Self(1u32 << 6);
    pub const RIGHT_CENTER: Self = Self(1u32 << 7);
    pub const CENTER_SURROUND: Self = Self(1u32 << 8); // WAVE: "Back Center"
    pub const LEFT_SURROUND_DIRECT: Self = Self(1u32 << 9);
    pub const RIGHT_SURROUND_DIRECT: Self = Self(1u32 << 10);
    pub const TOP_CENTER_SURROUND: Self = Self(1u32 << 11);
    pub const VERTICAL_HEIGHT_LEFT: Self = Self(1u32 << 12); // WAVE: "Top Front Left"
    pub const VERTICAL_HEIGHT_CENTER: Self = Self(1u32 << 13); // WAVE: "Top Front Center"
    pub const VERTICAL_HEIGHT_RIGHT: Self = Self(1u32 << 14); // WAVE: "Top Front Right"
    pub const TOP_BACK_LEFT: Self = Self(1u32 << 15);
    pub const TOP_BACK_CENTER: Self = Self(1u32 << 16);
    pub const TOP_BACK_RIGHT: Self = Self(1u32 << 17);
    pub const LEFT_TOP_FRONT: Self = Self::VERTICAL_HEIGHT_LEFT;
    pub const CENTER_TOP_FRONT: Self = Self::VERTICAL_HEIGHT_CENTER;
    pub const RIGHT_TOP_FRONT: Self = Self::VERTICAL_HEIGHT_RIGHT;
    pub const LEFT_TOP_MIDDLE: Self = Self(1u32 << 21);
    pub const CENTER_TOP_MIDDLE: Self = Self::TOP_CENTER_SURROUND;
    pub const RIGHT_TOP_MIDDLE: Self = Self(1u32 << 23);
    pub const LEFT_TOP_REAR: Self = Self(1u32 << 24);
    pub const CENTER_TOP_REAR: Self = Self(1u32 << 25);
    pub const RIGHT_TOP_REAR: Self = Self(1u32 << 26);
}