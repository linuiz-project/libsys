pub mod klog;
pub mod task;

#[repr(usize)]
#[derive(Debug, IntoPrimitive, TryFromPrimitive, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vector {
    KlogTrace = 0x100000,
    KlogDebug = 0x100001,
    KlogInfo = 0x100002,
    KlogWarn = 0x100003,
    KlogError = 0x100004,

    TaskKill = 0x200000,
    TaskDefer = 0x200001,
}

/// Converts a `code` and `value` from a kernel system call into a `Result<usize, TError>`.
fn into_result<TError: TryFrom<usize>>(code: usize, value: usize) -> Result<usize, TError> {
    match code {
        0 => Ok(value),
        code if let Ok(t_error) = TError::try_from(code) => Err(t_error),
        code => unreachable!("syscall returned invalid result code: {code}"),
    }
}

/// Syscall with 0 arguments.
#[allow(dead_code)]
fn syscall_0<TError: TryFrom<usize>>(vector: Vector) -> Result<usize, TError> {
    let code: usize;
    let value: usize;

    unsafe {
        cfg_select! {
            target_arch = "x86_64" => {
                core::arch::asm!(
                    "int 0x80",
                    inout("rsi") usize::from(vector) => value,
                    out("rdi") code,
                    options(preserves_flags)
                );
            }

            _ => { todo!() }
        }
    }

    into_result(code, value)
}

/// Syscall with 1 arguments.
#[allow(dead_code)]
fn syscall_1<TError: TryFrom<usize>>(vector: Vector, arg1: usize) -> Result<usize, TError> {
    let code: usize;
    let value: usize;

    unsafe {
        cfg_select! {
            target_arch = "x86_64" => {
                core::arch::asm!(
                    "int 0x80",
                    inout("rsi") usize::from(vector) => value,
                    inout("rdi") arg1 => code,
                    options(preserves_flags)
                );
            }

            _ => { todo!() }
        }
    }

    into_result(code, value)
}

/// Syscall with 2 arguments.
#[allow(dead_code)]
fn syscall_2<TError: TryFrom<usize>>(
    vector: Vector,
    arg1: usize,
    arg2: usize,
) -> Result<usize, TError> {
    let code: usize;
    let value: usize;

    unsafe {
        cfg_select! {
            target_arch = "x86_64" => {
                core::arch::asm!(
                    "int 0x80",
                    inout("rsi") usize::from(vector) => value,
                    inout("rdi") arg1 => code,
                    in("rax") arg2,
                    options(preserves_flags)
                );
            }

            _ => { todo!() }
        }
    }

    into_result(code, value)
}

/// Syscall with 3 arguments.
#[allow(dead_code)]
fn syscall_3<TError: TryFrom<usize>>(
    vector: Vector,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> Result<usize, TError> {
    let code: usize;
    let value: usize;

    unsafe {
        cfg_select! {
            target_arch = "x86_64" => {
                core::arch::asm!(
                    "int 0x80",
                    inout("rsi") usize::from(vector) => value,
                    inout("rdi") arg1 => code,
                    in("rax") arg2,
                    in("rcx") arg3,
                    options(preserves_flags)
                );
            }

            _ => { todo!() }
        }
    }

    into_result(code, value)
}

/// Syscall with 4 arguments.
#[allow(dead_code)]
fn syscall_4<TError: TryFrom<usize>>(
    vector: Vector,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> Result<usize, TError> {
    let code: usize;
    let value: usize;

    unsafe {
        cfg_select! {
            target_arch = "x86_64" => {
                core::arch::asm!(
                    "int 0x80",
                    inout("rsi") usize::from(vector) => value,
                    inout("rdi") arg1 => code,
                    in("rax") arg2,
                    in("rcx") arg3,
                    in("rdx") arg4,
                    options(preserves_flags)
                );
            }

            _ => { todo!() }
        }
    }

    into_result(code, value)
}
