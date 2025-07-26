#![no_std]
#![feature(step_trait)]
#![warn(clippy::pedantic)]

// In the future, there may be a platform where the pointer width does not
// exactly match CPU's the register width. In that case, we may need to
// introduce special types like `uptr` or `ureg` to handle the differing
// sizes of the CPU-internal structures.

#[macro_use]
extern crate thiserror;

#[macro_use]
extern crate num_enum;

pub mod address;
pub mod constants;
pub mod math;
pub mod syscall;
