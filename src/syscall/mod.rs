pub mod klog;
pub mod task;

#[repr(usize)]
#[derive(Debug, IntoPrimitive, TryFromPrimitive, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vector {
    KlogTrace = 0x100,
    KlogDebug = 0x101,
    KlogInfo = 0x102,
    KlogWarn = 0x103,
    KlogError = 0x104,

    TaskKill = 0x200,
    TaskDefer = 0x201,
}

fn into_result<TError: TryFrom<usize>>(code: usize, value: usize) -> Result<usize, TError> {
    match code {
        0 => Ok(value),
        code if let Ok(t_error) = TError::try_from(code) => Err(t_error),
        code => unreachable!("syscall returned invalid result code: {code}"),
    }
}

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
