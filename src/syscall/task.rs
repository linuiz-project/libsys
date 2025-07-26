use crate::syscall::{SyscallResult, Vector, syscall_0};

#[derive(Debug, Error)]
pub enum Error {
    #[error("there was no active task")]
    NoTask = 1,
}

impl From<SyscallResult> for Result<(), Error> {
    fn from(result: SyscallResult) -> Self {
        match result.code {
            0 => Ok(()),
            1 => Err(Error::NoTask),

            code => unreachable!("syscall returned invalid result code: {code}"),
        }
    }
}

/// Defers execution of the currently active task.
///
/// # Errors
///
/// - [`Error::NoTask`] if there's no active task on the current hardware thread.
pub fn defer() -> Result<(), Error> {
    syscall_0(Vector::TaskDefer).into()
}

/// Kills the currently active task.
///
/// # Errors
///
/// - [`Error::NoTask`] if there's no active task on the current hardware thread.
pub fn kill() -> Result<(), Error> {
    syscall_0(Vector::TaskKill).into()
}
