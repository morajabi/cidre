use std::ffi::c_void;

use crate::{at, av, blocks, define_obj_type, ns, objc};

#[doc(alias = "AVAudioVoiceProcessingSpeechActivityEvent")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum VPSpeechActivityEvent {
    /// Speech activity has started.
    Started = 0,
    /// Speech activity has ended.
    Ended = 1,
}

#[doc(alias = "AVAudioVoiceProcessingSpeechActivityEvent")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
#[repr(isize)]
pub enum VPOtherAudioDuckingLevel {
    #[default]
    Default = 0,
    Min = 10,
    Mid = 20,
    Max = 30,
}

#[doc(alias = "AVAudioVoiceProcessingOtherAudioDuckingConfiguration")]
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct VPOtherAudioDuckingCfg {
    pub enable_advanced_ducking: bool,
    pub ducking_level: VPOtherAudioDuckingLevel,
}

define_obj_type!(
    #[doc(alias = "AVAudioIONode")]
    pub IoNode(av::AudioNode)
);

impl IoNode {
    /// The presentation or hardware latency, applicable when the engine is rendering to/from an
    /// audio device.
    ///
    /// This corresponds to kAudioDevicePropertyLatency and kAudioStreamPropertyLatency.
    #[objc::msg_send(presentationLatency)]
    pub fn presentation_latency(&self) -> ns::TimeInterval;

    /// Indicates whether voice processing is enabled.
    #[objc::msg_send(isVoiceProcessingEnabled)]
    pub fn is_vp_enabled(&self) -> bool;

    #[objc::msg_send(setVoiceProcessingEnabled:error:)]
    pub unsafe fn set_vp_enabled_err<'ear>(
        &mut self,
        val: bool,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_vp_enabled<'ear>(&mut self, val: bool) -> Result<(), &'ear ns::Error> {
        let mut err = None;
        if unsafe { self.set_vp_enabled_err(val, &mut err) } {
            Ok(())
        } else {
            Err(unsafe { err.unwrap_unchecked() })
        }
    }
}

define_obj_type!(
    #[doc(alias = "AVAudioInputNode")]
    pub InputNode(IoNode)
);

impl InputNode {
    #[objc::msg_send(setManualRenderingInputPCMFormat:inputBlock:)]
    pub unsafe fn _set_manual_rendering_input_pcm_format(
        &mut self,
        format: &av::AudioFormat,
        input_block: *mut c_void,
    ) -> bool;

    pub fn set_manual_rendering_input_pcm_format<F>(
        &mut self,
        format: &av::AudioFormat,
        input_block: &'static mut blocks::Block<F>,
    ) -> Result<(), ()>
    where
        F: FnMut(av::AudioFrameCount) -> *const at::AudioBufList,
    {
        unsafe {
            if self._set_manual_rendering_input_pcm_format(format, input_block.as_mut_ptr()) {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    /// Bypass all processing done by the voice processing unit.
    ///
    /// Querying this property when voice processing is disabled will return false.
    #[objc::msg_send(isVoiceProcessingBypassed)]
    pub fn is_vp_bypassed(&self) -> bool;

    #[objc::msg_send(setVoiceProcessingBypassed:)]
    pub fn set_vp_bypassed(&mut self, val: bool);

    #[objc::msg_send(isVoiceProcessingAGCEnabled)]
    pub fn is_vp_agc_enabled(&self) -> bool;

    #[objc::msg_send(setVoiceProcessingAGCEnabled:)]
    pub fn set_vp_agc_enabled(&mut self, val: bool);

    #[objc::msg_send(isVoiceProcessingInputMuted)]
    pub fn is_vp_input_muted(&self) -> bool;

    #[objc::msg_send(setVoiceProcessingInputMuted:)]
    pub fn set_vp_input_muted(&mut self, val: bool);

    #[objc::msg_send(setMutedSpeechActivityEventListener:)]
    pub unsafe fn _set_muted_speech_activity_event_listener(&mut self, block: *mut c_void) -> bool;

    pub fn set_muted_speech_activity_event_listener<F>(
        &mut self,
        block: &'static mut blocks::Block<F>,
    ) -> Result<(), ()>
    where
        F: FnMut(VPSpeechActivityEvent),
    {
        unsafe {
            if self._set_muted_speech_activity_event_listener(block.as_mut_ptr()) {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    #[objc::msg_send(voiceProcessingOtherAudioDuckingConfiguration)]
    pub fn vp_other_audio_ducking_cfg(&self) -> VPOtherAudioDuckingCfg;

    #[objc::msg_send(setVoiceProcessingOtherAudioDuckingConfiguration:)]
    pub fn set_vp_other_audio_ducking_cfg(&mut self, val: VPOtherAudioDuckingCfg);
}

define_obj_type!(
    #[doc(alias = "AVAudioOutputNode")]
    pub OutputNode(IoNode)
);
