use crate::syscall::{SyscallResult, Vector, syscall};

#[derive(Debug, Error)]
pub enum Error {
    #[error("provided log string was not mapped into memory")]
    NotMapped,
}

impl From<SyscallResult> for Result<(), Error> {
    fn from(result: SyscallResult) -> Self {
        match result.code {
            0 => Ok(()),
            1 => Err(Error::NotMapped),

            code => unreachable!("kernel returned invalid result code: {code}"),
        }
    }
}

/// Logs a trace-level message to the kernel journal.
///
/// # Errors
///
/// - [`Error::NotMapped`] if the `str` is not mapped in the active address space.
pub fn trace(str: &str) -> Result<(), Error> {
    syscall(Vector::KlogTrace, str.as_ptr().addr(), str.len(), 0, 0).into()
}

/// Logs a debug-level message to the kernel journal.
///
/// # Errors
///
/// - [`Error::NotMapped`] if the `str` is not mapped in the active address space.
pub fn debug(str: &str) -> Result<(), Error> {
    syscall(Vector::KlogDebug, str.as_ptr().addr(), str.len(), 0, 0).into()
}

/// Logs a info-level message to the kernel journal.
///
/// # Errors
///
/// - [`Error::NotMapped`] if the `str` is not mapped in the active address space.
pub fn info(str: &str) -> Result<(), Error> {
    syscall(Vector::KlogInfo, str.as_ptr().addr(), str.len(), 0, 0).into()
}

/// Logs a warn-level message to the kernel journal.
///
/// # Errors
///
/// - [`Error::NotMapped`] if the `str` is not mapped in the active address space.
pub fn warn(str: &str) -> Result<(), Error> {
    syscall(Vector::KlogWarn, str.as_ptr().addr(), str.len(), 0, 0).into()
}

/// Logs a error-level message to the kernel journal.
///
/// # Errors
///
/// - [`Error::NotMapped`] if the `str` is not mapped in the active address space.
pub fn error(str: &str) -> Result<(), Error> {
    syscall(Vector::KlogError, str.as_ptr().addr(), str.len(), 0, 0).into()
}
