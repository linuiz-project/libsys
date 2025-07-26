use crate::syscall::{Vector, syscall_0};

#[repr(usize)]
#[derive(Debug, Error, IntoPrimitive, TryFromPrimitive)]
pub enum Error {
    #[error("there was no active task")]
    NoTask = 1,
}

/// Defers execution of the currently active task.
///
/// # Errors
///
/// - [`Error::NoTask`] if there's no active task on the current hardware thread.
pub fn defer() -> Result<(), Error> {
    syscall_0(Vector::TaskDefer).map(|_| ())
}

/// Kills the currently active task.
///
/// # Errors
///
/// - [`Error::NoTask`] if there's no active task on the current hardware thread.
pub fn kill() -> Result<(), Error> {
    syscall_0(Vector::TaskKill).map(|_| ())
}
