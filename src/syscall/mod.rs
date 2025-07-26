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

#[allow(dead_code)]
struct SyscallResult {
    code: isize,
    value: usize,
}

#[allow(dead_code)]
fn syscall_0(vector: Vector) -> SyscallResult {
    let code: isize;
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

    SyscallResult { code, value }
}

#[allow(dead_code)]
fn syscall_1(vector: Vector, arg1: usize) -> SyscallResult {
    let code: isize;
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

    SyscallResult { code, value }
}

#[allow(dead_code)]
fn syscall_2(vector: Vector, arg1: usize, arg2: usize) -> SyscallResult {
    let code: isize;
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

    SyscallResult { code, value }
}

#[allow(dead_code)]
fn syscall_3(vector: Vector, arg1: usize, arg2: usize, arg3: usize) -> SyscallResult {
    let code: isize;
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

    SyscallResult { code, value }
}

#[allow(dead_code)]
fn syscall_4(vector: Vector, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> SyscallResult {
    let code: isize;
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

    SyscallResult { code, value }
}
