pub use crate::cat::audio::*;

mod format;
pub use format::asbd_props;
pub use format::channel_layout_props;
pub use format::id3_props;
pub use format::BalanceFade;
pub use format::BalanceFadeType;
pub use format::ExtendedFormatInfo;
pub use format::FormatInfo;
pub use format::PanningInfo;
pub use format::PanningMode;
pub use format::Prop as FormatProp;

mod converter;
pub use converter::errors;
pub use converter::Converter;
pub use converter::ConverterRef;
pub use converter::DitherAlgorithm;
pub use converter::PrimeMethod as ConverterPrimeMethod;

mod component;
pub use component::Component;
pub use component::ComponentInstance;
pub use component::ComponentInstanceRef;
pub use component::Desc as ComponentDesc;
pub use component::Flags as ComponentFlags;
pub use component::InstantiationOpts as ComponentInstantiationOpts;

mod unit;

mod file;
pub use file::errors as file_errors;
pub use file::FileID;
pub use file::FileTypeID;
pub use file::Flags as FileFlags;
pub use file::Permissions as FilePermissions;
pub use file::PropertyID as FilePropertyID;

mod codec;
pub use codec::quality as codec_quality;
pub use codec::BitRateControlMode as CodecBitRateControlMode;
pub use codec::Codec;
pub use codec::CodecRef;
pub use codec::GlobalProperty as CodecGlobalProperty;
pub use codec::InstanceProperty as CodecInstanceProperty;
pub use codec::MagicCookieInfo as CodecMagicCookieInfo;
pub use codec::ProduceOutputPacketStatus as CodecProduceOutputPacketStatus;
pub use codec::DECODER_COMPONENT_TYPE;
pub use codec::ENCODER_COMPONENT_TYPE;
pub use codec::UNITY_CODEC_COMPONENT_TYPE;
