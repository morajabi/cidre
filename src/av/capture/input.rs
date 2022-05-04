use crate::{cf, define_obj_type, objc::Id};

define_obj_type!(Input(Id));
define_obj_type!(Port(Id));

impl Input {
    pub fn ports(&self) -> &cf::ArrayOf<Port> {
        unsafe { rsel_ports(self) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn rsel_ports(input: &Input) -> &cf::ArrayOf<Port>;
}

impl Port {
    pub fn input(&self) -> &Input {
        unsafe { rsel_input(self) }
    }

    pub fn enabled(&self) -> bool {
        unsafe { rsel_isEnabled(self) }
    }

    pub fn set_enabled(&self, value: bool) {
        unsafe { wsel_setEnabled(self, value) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn rsel_input(port: &Port) -> &Input;
}

#[link(name = "common", kind = "static")]
extern "C" {
    fn rsel_isEnabled(id: &Id) -> bool;
    fn wsel_setEnabled(id: &Id, value: bool);
}

pub mod port_notifications {
    use crate::cf;

    pub fn format_description_did_change() -> &'static cf::NotificationName {
        unsafe { AVCaptureInputPortFormatDescriptionDidChangeNotification }
    }

    #[link(name = "AVFoundation", kind = "framework")]
    extern "C" {
        static AVCaptureInputPortFormatDescriptionDidChangeNotification:
            &'static cf::NotificationName;
    }
}