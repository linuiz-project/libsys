pub mod klog;
pub mod task;

use core::ffi::c_void;
use num_enum::TryFromPrimitive;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, Hash)]
pub enum Vector {
    KlogInfo = 0x100,
    KlogError = 0x101,
    KlogDebug = 0x102,
    KlogTrace = 0x103,

    TaskExit = 0x200,
    TaskYield = 0x201,
}


const_assert!(size_of::<Result>() == size_of::<(u64, u64)>());

#[repr(u32)]
#[derive(Debug, TryFromPrimitive, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidVector = 0x10000,
    InvalidPtr = 0x20000,
    InvalidUtf8 = 0x30000,

    UnmappedMemory = 0x40000,

    NoActiveTask = 0x50000,
}

impl From<core::str::Utf8Error> for Error {
    fn from(_: core::str::Utf8Error) -> Self {
        Self::InvalidUtf8
    }
}
