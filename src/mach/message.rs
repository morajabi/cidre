use std::ffi::c_void;

use crate::define_options;

use super::{Boolean, KernReturn, Port, PortName};

pub type MsgBits = u32;
pub type MsgSize = i32;
pub type MsgId = i32;
pub type MsgPriority = u32;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum TypeName {
    MoveRecieve = 16,
    MoveSend = 17,
    MoveSendOnce = 18,
    MoveCopySend = 19,
    MakeSend = 20,
    MakeSendOnce = 21,
    CopyReceive = 22,
    DisposeReceive = 24,
    DisposeSend = 25,
    DisposeSendOnce = 26,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum CopyOptions {
    PhysicalCopy = 0,
    VirtualCopy = 1,
    Allocate = 2,
    Overwrite = 3,
    KallocCopy = 4,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum GuardFlags {
    None,
    ImmovableReceive,
    UnguardedOnSend,
}

impl GuardFlags {
    pub const MASK: u32 = 3;
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum DescriptorType {
    Port,

    /// Out of line
    OOL,
    OOLPorts,
    OOLVolatile,
    GuardedPort,
}

impl DescriptorType {
    pub const MAX: Self = Self::GuardedPort;
}

#[repr(C)]
pub struct TypeDescriptor {
    pub pad1: u32,
    pub pad2: MsgSize,
    pub pad3: u32,
    pub type_: DescriptorType,
}

#[repr(C)]
pub struct PortDescriptor {
    pub name: Port,
    pub pad1: MsgSize,
    pub pad2: u32,
    pub disposition: TypeName,
    pub type_: DescriptorType,
}

#[repr(C)]
pub struct OOLDescriptor32 {
    pub address: u32,
    pub size: MsgSize,
    pub deallocate: Boolean,
    pub pad1: u32,
    pub type_: DescriptorType,
}

#[repr(C)]
pub struct OOLDescriptor64 {
    pub address: u64,
    pub size: MsgSize,
    pub deallocate: Boolean,
    pub pad1: u32,
    pub type_: DescriptorType,
}

#[repr(C)]
pub struct OOLDescriptor {
    pub address: *mut c_void,
    pub deallocate: Boolean,
    pub copy: CopyOptions,
    pub pad1: u32,
    pub type_: DescriptorType,
    pub size: MsgSize,
}

#[repr(C)]
pub struct Body {
    pub descriptor_count: MsgSize,
}

#[repr(C)]
pub struct Header {
    pub bits: MsgBits,
    pub size: MsgSize,
    pub remote_port: Port,
    pub local_port: Port,
    pub voucher_port: PortName,
    pub id: MsgId,
}

#[repr(C)]
pub struct Base {
    pub header: Header,
    pub body: Body,
}

#[repr(transparent)]
pub struct Return(pub KernReturn);

impl Return {
    pub const SUCCESS: Self = Self(KernReturn(0));

    pub const MASK: Self = Self(KernReturn(0x00003e00));

    /// No room in IPC name space for another capability name.
    pub const IPC_SPACE: Self = Self(KernReturn(0x00002000));

    ///  No room in VM address space for out-of-line memory.
    pub const VM_SPACE: Self = Self(KernReturn(0x00001000));

    /// Kernel resource shortage handling an IPC capability.
    pub const IPC_KERNEL: Self = Self(KernReturn(0x00000800));

    /// Kernel resource shortage handling out-of-line memory.
    pub const VM_KERNEL: Self = Self(KernReturn(0x00000400));

    /// Thread is waiting to send.  (Internal use only.)
    pub const SEND_IN_PROGRESS: Self = Self(KernReturn(0x10000001));

    /// Bogus in-line data.
    pub const SEND_INVALID_DATA: Self = Self(KernReturn(0x10000002));

    /// Bogus destination port.
    pub const SEND_INVALID_DEST: Self = Self(KernReturn(0x10000003));

    /// Message not sent before timeout expired.
    pub const SEND_TIMED_OUT: Self = Self(KernReturn(0x10000004));

    /// Bogus voucher port.
    pub const SEND_INVALID_VOUCHER: Self = Self(KernReturn(0x10000005));

    /// Software interrupt.
    pub const SEND_INTERRUPTED: Self = Self(KernReturn(0x10000007));

    /// Data doesn't contain a complete message.
    pub const SEND_SEND_MSG_TOO_SMALL: Self = Self(KernReturn(0x10000008));

    ///  Bogus reply port.
    pub const SEND_INVALID_REPLY: Self = Self(KernReturn(0x10000009));

    /// Bogus port rights in the message body.
    pub const SEND_INVALID_RIGHT: Self = Self(KernReturn(0x1000000a));

    /// Bogus notify port argument.
    pub const SEND_INVALID_NOTIFY: Self = Self(KernReturn(0x1000000b));

    /// Invalid out-of-line memory pointer.
    pub const SEND_INVALID_MEMORY: Self = Self(KernReturn(0x1000000c));

    /// No message buffer is available.
    pub const SEND_NO_BUFFER: Self = Self(KernReturn(0x1000000d));

    /// Send is too large for port
    pub const SEND_TOO_LARGE: Self = Self(KernReturn(0x1000000e));

    /// Invalid msg-type specification.
    pub const SEND_INVALID_TYPE: Self = Self(KernReturn(0x1000000f));

    ///  A field in the header had a bad value.
    pub const SEND_INVALID_HEADER: Self = Self(KernReturn(0x10000010));

    /// The trailer to be sent does not match kernel format.
    pub const SEND_INVALID_TRAILER: Self = Self(KernReturn(0x10000011));

    /// The sending thread context did not match the context on the dest port
    pub const SEND_INVALID_CONTEXT: Self = Self(KernReturn(0x10000012));

    ///  Send options are invalid.
    pub const SEND_INVALID_OPTIONS: Self = Self(KernReturn(0x10000013));

    ///  compatibility: no longer a returned error
    pub const SEND_INVALID_RT_OOL_SIZE: Self = Self(KernReturn(0x10000015));

    /// The destination port doesn't accept ports in body
    pub const SEND_NO_GRANT_DEST: Self = Self(KernReturn(0x10000016));

    /// Message send was rejected by message filter
    pub const SEND_MSG_FILTERED: Self = Self(KernReturn(0x10000017));

    /// Message auxiliary data is too small
    pub const SEND_AUX_TOO_SMALL: Self = Self(KernReturn(0x10000018));

    /// Message auxiliary data is too large
    pub const SEND_SEND_AUX_TOO_LARGE: Self = Self(KernReturn(0x10000019));

    /// Thread is waiting for receive.  (Internal use only.)
    pub const RCV_IN_PROGRESS: Self = Self(KernReturn(0x10004001));

    /// Bogus name for receive port/port-set.
    pub const RCV_INVALID_NAME: Self = Self(KernReturn(0x10004002));

    /// Didn't get a message within the timeout value.
    pub const RCV_TIMED_OUT: Self = Self(KernReturn(0x10004003));

    /// Message buffer is not large enough for inline data.
    pub const RCV_TOO_LARGE: Self = Self(KernReturn(0x10004004));

    /// Software interrupt.    
    pub const RCV_INTERRUPTED: Self = Self(KernReturn(0x10004005));

    /// compatibility: no longer a returned error
    pub const RCV_PORT_CHANGED: Self = Self(KernReturn(0x10004006));

    /// Bogus notify port argument.
    pub const RCV_INVALID_NOTIFY: Self = Self(KernReturn(0x10004007));

    /// Bogus message buffer for inline data.
    pub const RCV_INVALID_DATA: Self = Self(KernReturn(0x10004008));

    /// Port/set was sent away/died during receive.
    pub const RCV_PORT_DIED: Self = Self(KernReturn(0x10004009));

    /// compatibility: no longer a returned error
    pub const RCV_IN_SET: Self = Self(KernReturn(0x1000400a));

    /// Error receiving message header.  See special bits.
    pub const RCV_HEADER_ERROR: Self = Self(KernReturn(0x1000400b));

    /// Error receiving message body.  See special bits.
    pub const RCV_BODY_ERROR: Self = Self(KernReturn(0x1000400c));

    /// Invalid msg-type specification in scatter list.
    pub const RCV_INVALID_TYPE: Self = Self(KernReturn(0x1000400d));

    /// Out-of-line overwrite region is not large enough
    pub const RCV_SCATTER_SMALL: Self = Self(KernReturn(0x1000400e));

    /// trailer type or number of trailer elements not supported
    pub const RCV_INVALID_TRAILER: Self = Self(KernReturn(0x1000400f));

    /// Waiting for receive with timeout. (Internal use only.)
    pub const RCV_IN_PROGRESS_TIMED: Self = Self(KernReturn(0x10004011));

    /// invalid reply port used in a STRICT_REPLY message    
    pub const RCV_INVALID_REPLY: Self = Self(KernReturn(0x10004012));

    /// invalid receive arguments, receive has not started
    pub const RCV_INVALID_ARGUMENTS: Self = Self(KernReturn(0x10004013));
}

define_options!(MsgOption(i32));

#[repr(transparent)]
pub struct Timeout(pub u32);

impl Timeout {
    pub const NONE: Self = Self(0);
}

#[inline]
pub fn msg(
    msg: &mut Header,
    option: MsgOption,
    send_size: MsgSize,
    rcv_size: MsgSize,
    rcv_name: PortName,
    timeout: Timeout,
    notify: PortName,
) -> Return {
    unsafe { mach_msg(msg, option, send_size, rcv_size, rcv_name, timeout, notify) }
}

extern "C" {
    fn mach_msg(
        msg: &mut Header,
        option: MsgOption,
        send_size: MsgSize,
        rcv_size: MsgSize,
        rcv_name: PortName,
        timeout: Timeout,
        notify: PortName,
    ) -> Return;
}
