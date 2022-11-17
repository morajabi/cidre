use std::ffi::c_void;
use std::intrinsics::transmute;
use std::ptr::NonNull;

use crate::{cf, define_cf_type, define_options};

define_cf_type!(Socket(cf::Type));

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Error(cf::Index);

impl Error {
    pub const SUCCESS: Self = Self(0);
    pub const ERROR: Self = Self(-1);
    pub const TIMEOUT: Self = Self(-2);
}

#[derive(Debug)]
#[repr(C)]
pub struct Signature {
    pub protocol_family: i32,
    pub socket_type: i32,
    pub protocol: i32,
    pub address: cf::Retained<cf::Data>,
}

define_options!(CallBackType(usize));

impl CallBackType {
    pub const NO: Self = Self(0);
    pub const READ: Self = Self(1);
    pub const ACCEPT: Self = Self(2);
    pub const DATA: Self = Self(3);
    pub const CONNECT: Self = Self(4);
    pub const WRITE: Self = Self(8);
}

define_options!(Flags(usize));

impl Flags {
    pub const AUTOMATICALLY_REENABLE_READ_CALL_BACK: Self = Self(1);
    pub const AUTOMATICALLY_REENABLE_ACCEPT_CALL_BACK: Self = Self(2);
    pub const AUTOMATICALLY_REENABLE_DATA_CALL_BACK: Self = Self(3);
    pub const AUTOMATICALLY_REENABLE_WRITE_CALL_BACK: Self = Self(8);
    pub const LEAVE_ERRORS: Self = Self(64);
    pub const CLOSE_ON_INVALIDATE: Self = Self(128);
}

pub type CallBack<T> = extern "C" fn(
    s: &Socket,
    cb_type: CallBackType,
    address: &cf::Data,
    data: *const u8,
    info: *mut T,
);

#[repr(C)]
pub struct Context<T> {
    pub version: cf::Index,
    pub info: *mut T,
    pub retain: Option<extern "C" fn(info: *const T)>,
    pub release: Option<extern "C" fn(info: *const T)>,
    pub copy_description: Option<extern "C" fn(info: *const T) -> Option<cf::Retained<cf::String>>>,
}

pub type NativeHandle = i32;

impl Socket {
    pub unsafe fn create_in<T>(
        protocol_family: i32,
        socket_type: i32,
        protocol: i32,
        cb_types: CallBackType,
        callout: CallBack<T>,
        context: Option<NonNull<Context<c_void>>>,
        allocator: Option<&cf::Allocator>,
    ) -> Option<cf::Retained<Socket>> {
        CFSocketCreate(
            allocator,
            protocol_family,
            socket_type,
            protocol,
            cb_types,
            transmute(callout),
            context,
        )
    }
    pub unsafe fn create_with_native_in<T>(        
        sock: NativeHandle,
        cb_types: CallBackType,
        callout: CallBack<T>,
        context: Option<NonNull<Context<c_void>>>,
        allocator: Option<&cf::Allocator>,
    ) -> Option<cf::Retained<Socket>> {
        CFSocketCreateWithNative(
            allocator,
            sock,
            cb_types,
            transmute(callout),
            context,
        )
    }

    pub unsafe fn create_with_native<T>(
        sock: NativeHandle,
        cb_types: CallBackType,
        callout: CallBack<T>,
        context: Option<NonNull<Context<c_void>>>,
    ) -> Option<cf::Retained<Socket>> {
        Self::create_with_native_in(sock, cb_types, callout, context, None)
    }

    pub fn native(&self) -> NativeHandle {
        unsafe { CFSocketGetNative(self) }
    }

    pub fn invalidate(&self) {
        unsafe { CFSocketInvalidate(self) }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { CFSocketIsValid(self) }
    }

    pub fn create_runloop_source_in(
        &self,        
        order: cf::Index,
        allocator: Option<&cf::Allocator>,
    ) -> Option<cf::Retained<cf::RunLoopSource>> {
        unsafe { CFSocketCreateRunLoopSource(allocator, self, order) }
    }

    pub fn create_runloop_source(
        &self,
        order: cf::Index,
    ) -> Option<cf::Retained<cf::RunLoopSource>> {
        self.create_runloop_source_in(order, None)
    }

    pub fn flags(&self) -> Flags {
        unsafe { CFSocketGetSocketFlags(self) }
    }

    pub fn set_flags(&self, flags: Flags) {
        unsafe { CFSocketSetSocketFlags(self, flags) }
    }

    pub fn enable_callbacks(&self, cb_types: CallBackType) {
        unsafe { CFSocketEnableCallBacks(self, cb_types) }
    }

    pub fn disable_callbacks(&self, cb_types: CallBackType) {
        unsafe { CFSocketDisableCallBacks(self, cb_types) }
    }

    pub fn send_data(
        &self,
        address: Option<&cf::Data>,
        data: &cf::Data,
        timeout: cf::TimeInterval,
    ) -> Error {
        unsafe { CFSocketSendData(self, address, data, timeout) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFSocketCreate(
        allocator: Option<&cf::Allocator>,
        protocol_family: i32,
        socket_type: i32,
        protocol: i32,
        cb_types: CallBackType,
        callout: CallBack<c_void>,
        context: Option<NonNull<Context<c_void>>>,
    ) -> Option<cf::Retained<Socket>>;

    fn CFSocketCreateWithNative(
        allocator: Option<&cf::Allocator>,
        sock: NativeHandle,
        cb_types: CallBackType,
        callout: CallBack<c_void>,
        context: Option<NonNull<Context<c_void>>>,
    ) -> Option<cf::Retained<Socket>>;

    fn CFSocketGetNative(s: &Socket) -> NativeHandle;

    fn CFSocketInvalidate(s: &Socket);

    fn CFSocketIsValid(s: &Socket) -> bool;

    fn CFSocketCreateRunLoopSource(
        allocator: Option<&cf::Allocator>,
        s: &Socket,
        order: cf::Index,
    ) -> Option<cf::Retained<cf::RunLoopSource>>;

    fn CFSocketGetSocketFlags(s: &Socket) -> Flags;
    fn CFSocketSetSocketFlags(s: &Socket, flags: Flags);

    fn CFSocketDisableCallBacks(s: &Socket, cb_types: CallBackType);
    fn CFSocketEnableCallBacks(s: &Socket, cb_types: CallBackType);

    fn CFSocketSendData(
        s: &Socket,
        address: Option<&cf::Data>,
        data: &cf::Data,
        timeout: cf::TimeInterval,
    ) -> Error;

}
