use crate::syscall::{Vector, syscall_2};

#[repr(usize)]
#[derive(Debug, Error, IntoPrimitive, TryFromPrimitive)]
pub enum Error {
    #[error("provided log string was not mapped into memory")]
    NotMapped = 1,
}

/// Logs a trace-level message to the kernel journal.
///
/// # Errors
///
/// - [`Error::NotMapped`] if the `str` is not mapped in the active address space.
pub fn trace(str: &str) -> Result<(), Error> {
    syscall_2(Vector::KlogTrace, str.as_ptr().addr(), str.len()).map(|_| ())
}

/// Logs a debug-level message to the kernel journal.
///
/// # Errors
///
/// - [`Error::NotMapped`] if the `str` is not mapped in the active address space.
pub fn debug(str: &str) -> Result<(), Error> {
    syscall_2(Vector::KlogDebug, str.as_ptr().addr(), str.len()).map(|_| ())
}

/// Logs a info-level message to the kernel journal.
///
/// # Errors
///
/// - [`Error::NotMapped`] if the `str` is not mapped in the active address space.
pub fn info(str: &str) -> Result<(), Error> {
    syscall_2(Vector::KlogInfo, str.as_ptr().addr(), str.len()).map(|_| ())
}

/// Logs a warn-level message to the kernel journal.
///
/// # Errors
///
/// - [`Error::NotMapped`] if the `str` is not mapped in the active address space.
pub fn warn(str: &str) -> Result<(), Error> {
    syscall_2(Vector::KlogWarn, str.as_ptr().addr(), str.len()).map(|_| ())
}

/// Logs a error-level message to the kernel journal.
///
/// # Errors
///
/// - [`Error::NotMapped`] if the `str` is not mapped in the active address space.
pub fn error(str: &str) -> Result<(), Error> {
    syscall_2(Vector::KlogError, str.as_ptr().addr(), str.len()).map(|_| ())
}
