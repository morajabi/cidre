use crate::{arc, av, cm, define_cls, define_obj_type, ns, objc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum Status {
    /// Indicates that the status of the asset reader is not currently known.
    Unknown = 0,
    /// Indicates that the asset reader is successfully reading samples from its asset.
    Reading = 1,
    /// Indicates that the asset reader has successfully read all of the samples in its time range.
    Completed = 2,
    /// Indicates that the asset reader can no longer read samples from its asset because of an error. The error is described by the value of the asset reader's error property.
    Failed = 3,
    /// Indicates that the asset reader can no longer read samples because reading was canceled with the cancelReading method.
    Cancelled = 4,
}

define_obj_type!(pub Reader(ns::Id));

impl arc::A<Reader> {
    #[objc::msg_send(initWithAsset:error:)]
    pub fn init_with_assert_err<'ar>(
        self,
        asset: &av::Asset,
        error: *mut Option<&'ar ns::Error>,
    ) -> Option<arc::R<Reader>>;
}

impl Reader {
    define_cls!(AV_ASSET_READER);

    pub fn with_asset<'ar>(asset: &av::Asset) -> Result<arc::R<Reader>, &'ar ns::Error> {
        let mut error = None;
        unsafe {
            if let Some(reader) = Self::alloc().init_with_assert_err(asset, &mut error) {
                Ok(reader)
            } else {
                Err(error.unwrap_unchecked())
            }
        }
    }

    #[objc::msg_send(addOutput:)]
    pub unsafe fn add_output_throws(&mut self, output: &av::AssetReaderOutput);

    #[inline]
    pub fn add_output<'ar>(
        &mut self,
        output: &av::AssetReaderOutput,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.add_output_throws(output) })
    }

    /// Prepares the receiver for reading sample buffers from the asset.
    ///
    /// This method validates the entire collection of settings for outputs for tracks, for audio mixing, and for video composition and initiates reading from the receiver's asset.
    /// If this method returns `false`, clients can determine the nature of the failure by checking the value of the status and error properties.
    ///
    /// This method throws an exception if reading has already started (`status` has progressed beyond AVAssetReaderStatusUnknown).
    #[objc::msg_send(startReading)]
    pub fn start_reading(&mut self) -> bool;

    /// Cancels any background work and prevents the receiver's outputs from reading more samples.
    ///
    /// Clients that want to stop reading samples from the receiver before reaching the end of its time range should call this method to stop any background read ahead operations that the may have been in progress.
    ///
    /// This method should not be called concurrently with any calls to -[AVAssetReaderOutput copyNextSampleBuffer].
    #[objc::msg_send(cancelReading)]
    pub fn cancel_reading(&mut self);

    #[objc::msg_send(canAddOutput:)]
    pub fn can_add_output(&self, output: &av::AssetReaderOutput) -> bool;

    #[objc::msg_send(error)]
    pub fn error(&self) -> Option<&ns::Error>;

    #[objc::msg_send(status)]
    pub fn status(&self) -> Status;

    #[objc::msg_send(timeRange)]
    pub fn time_range(&self) -> cm::TimeRange;

    #[objc::msg_send(setTimeRange:)]
    pub fn set_time_range(&mut self, value: cm::TimeRange);

    #[objc::msg_send(outputs)]
    pub fn outputs(&self) -> &ns::Array<av::AssetReaderOutput>;
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_ASSET_READER: &'static objc::Class<Reader>;
}
