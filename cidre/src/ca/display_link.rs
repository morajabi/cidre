use crate::{arc, ca, cf, define_obj_type, ns, objc};

define_obj_type!(DisplayLink(ns::Id), CA_DISPLAY_LINK);

#[objc::obj_trait]
pub trait Target: objc::Obj {
    #[objc::msg_send(onDisplayLink:)]
    fn on_display_link(&mut self, link: &DisplayLink);
}

impl DisplayLink {
    #[objc::cls_msg_send(displayLinkWithTarget:selector:)]
    pub fn with_target_selector_ar(target: &ns::Id, selector: &objc::Sel) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_target_selector(target: &ns::Id, selector: &objc::Sel) -> arc::R<Self>;

    pub fn with<D: TargetImpl>(target: &D) -> arc::R<Self> {
        unsafe {
            Self::with_target_selector(
                std::mem::transmute(target),
                D::sel_on_display_link().unwrap_unchecked(),
            )
        }
    }

    /// Adds the receiver to the given run-loop and mode. Unless paused, it
    /// will fire every vsync until removed. Each object may only be added
    /// to a single run-loop, but it may be added in multiple modes at once.
    /// While added to a run-loop it will implicitly be retained.
    #[objc::msg_send(addToRunLoop:forMode:)]
    pub fn add_to_run_loop_for_mode(&self, runloop: &ns::RunLoop, mode: &ns::RunLoopMode);

    /// Removes the receiver from the given mode of the runloop. This will
    /// implicitly release it when removed from the last mode it has been
    /// registered for.
    #[objc::msg_send(removeFromRunLoop:forMode:)]
    pub fn remove_from_run_loop_for_mode(&self, runloop: &ns::RunLoop, mode: &ns::RunLoopMode);

    /// Removes the object from all runloop modes (releasing the receiver if
    /// it has been implicitly retained) and releases the 'target' object
    #[objc::msg_send(invalidate)]
    pub fn invalidate(&self);

    #[objc::msg_send(timestamp)]
    pub fn timestamp(&self) -> cf::TimeInterval;

    #[objc::msg_send(duration)]
    pub fn duration(&self) -> cf::TimeInterval;

    /// The time interval that represents when the next frame displays.
    #[objc::msg_send(targetTimestamp)]
    pub fn target_timestamp(&self) -> cf::TimeInterval;

    /// Defines the range of desired callback rate in frames-per-second for this
    /// display link. If the range contains the same minimum and maximum frame rate,
    /// this property is identical as preferredFramesPerSecond. Otherwise, the actual
    /// callback rate will be dynamically adjusted to better align with other
    /// animation sources.
    #[objc::msg_send(preferredFrameRateRange)]
    pub fn preferred_frame_rate_range(&self) -> ca::FrameRateRange;

    #[objc::msg_send(setPreferredFrameRateRange:)]
    pub fn set_preferred_frame_rate_range(&self, value: ca::FrameRateRange);
}

#[link(name = "ca", kind = "static")]
extern "C" {
    static CA_DISPLAY_LINK: &'static objc::Class<ns::Id>;
}
