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

#[cfg(target_arch = "x86_64")]
fn syscall(vector: Vector, arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> SyscallResult {
    let code: isize;
    let value: usize;

    unsafe {
        core::arch::asm!(
            "int 0x80",
            inout("rsi") usize::from(vector) => value,
            inout("rdi") arg0 => code,
            in("rax") arg1,
            in("rcx") arg2,
            in("rdx") arg3,
            options(preserves_flags)
        );

        SyscallResult { code, value }
    }
}
