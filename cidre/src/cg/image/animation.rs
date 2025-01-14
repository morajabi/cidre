use std::ffi::c_void;

use crate::{blocks, cf, cg, define_cf_type, os};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct Status(os::Status);

impl Status {
    /// NULL or invalid parameter passed to API
    pub const PARAMAMETER_ERROR: Self = Self(os::Status(-22140));

    /// An image cannot be read from the given source
    pub const CORRUPT_INPUT_IMAGE: Self = Self(os::Status(-22141));

    /// The image format is not applicable to animation
    pub const UNSUPPORTED_FORMAT: Self = Self(os::Status(-22142));

    /// An image can be read from the given source, but it is incomplete
    pub const INCOMPLETE_INPUT_IMAGE: Self = Self(os::Status(-22143));

    /// A required resource could not be created
    pub const ALLOCATION_FAILURE: Self = Self(os::Status(-22143));
}

define_cf_type!(OptKey(cf::String));

impl OptKey {
    /// Starts the animation at the given index. Defaults to 0
    /// Value is a 'cf::Number'
    #[doc(alias = "kCGImageAnimationStartIndex")]
    pub fn start_index() -> &'static OptKey {
        unsafe { kCGImageAnimationStartIndex }
    }

    /// The value of this key overrides the `delay time' specified by the image
    /// Value is a 'cf::Number' of 'cf::NumberType::DOUBLE'
    #[doc(alias = "kCGImageAnimationDelayTime")]
    pub fn delay_time() -> &'static OptKey {
        unsafe { kCGImageAnimationDelayTime }
    }

    /// The value of this key overrides the `loop count' specified by the image
    /// Value is a 'cf::Number';  'cf::Number::positive_infinity()' may be used.
    #[doc(alias = "kCGImageAnimationLoopCount")]
    pub fn loop_count() -> &'static OptKey {
        unsafe { kCGImageAnimationLoopCount }
    }
}

#[cfg(feature = "blocks")]
#[doc(alias = "CGAnimateImageAtURLWithBlock")]
#[inline]
pub fn animate_image_at_url_with_block<F>(
    url: &cf::Url,
    options: Option<&cf::DictionaryOf<OptKey, cf::Number>>,
    block: &'static mut blocks::Block<F>,
) -> os::Status
where
    F: FnMut(usize, &cg::Image, *mut bool) -> os::Status,
{
    unsafe { CGAnimateImageAtURLWithBlock(url, options, block.as_mut_ptr()) }
}

#[cfg(feature = "blocks")]
#[doc(alias = "CGAnimateImageAtURLWithBlock")]
#[inline]
pub fn animate_image_at_url<F>(
    url: &cf::Url,
    options: Option<&cf::DictionaryOf<OptKey, cf::Number>>,
    block: &'static mut blocks::Block<F>,
) -> Result<(), os::Status>
where
    F: FnMut(usize, &cg::Image, *mut bool) -> os::Status,
{
    unsafe { CGAnimateImageAtURLWithBlock(url, options, block.as_mut_ptr()).result() }
}

#[cfg(feature = "blocks")]
#[doc(alias = "CGAnimateImageDataWithBlock")]
#[inline]
pub fn animate_image_data_with_block<F>(
    data: &cf::Data,
    options: Option<&cf::DictionaryOf<OptKey, cf::Number>>,
    block: &'static mut blocks::Block<F>,
) -> os::Status
where
    F: FnMut(usize, &cg::Image, *mut bool) -> os::Status,
{
    unsafe { CGAnimateImageDataWithBlock(data, options, block.as_mut_ptr()) }
}

#[cfg(feature = "blocks")]
#[doc(alias = "CGAnimateImageDataWithBlock")]
#[inline]
pub fn animate_image_data<F>(
    data: &cf::Data,
    options: Option<&cf::DictionaryOf<OptKey, cf::Number>>,
    block: &'static mut blocks::Block<F>,
) -> Result<(), os::Status>
where
    F: FnMut(usize, &cg::Image, *mut bool) -> os::Status,
{
    unsafe { CGAnimateImageDataWithBlock(data, options, block.as_mut_ptr()).result() }
}

#[link(name = "ImageIO", kind = "framework")]
extern "C" {
    static kCGImageAnimationStartIndex: &'static OptKey;
    static kCGImageAnimationDelayTime: &'static OptKey;
    static kCGImageAnimationLoopCount: &'static OptKey;

    fn CGAnimateImageAtURLWithBlock(
        url: &cf::Url,
        options: Option<&cf::DictionaryOf<OptKey, cf::Number>>,
        block: *mut c_void,
    ) -> os::Status;

    fn CGAnimateImageDataWithBlock(
        data: &cf::Data,
        options: Option<&cf::DictionaryOf<OptKey, cf::Number>>,
        block: *mut c_void,
    ) -> os::Status;

}
