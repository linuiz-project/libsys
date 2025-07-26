#![no_std]
#![feature(step_trait)]

// In the future, there may be a platform where the pointer width does not
// exactly match CPU's the register width. In that case, we may need to
// introduce special types like `uptr` or `ureg` to handle the differing
// sizes of the CPU-internal structures.

mod address;
pub use address::*;

mod constants;
pub use constants::*;

pub mod syscall;

#[macro_use]
extern crate static_assertions;

use core::{num::NonZero, ops::Shr};

pub const fn align_up(value: usize, alignment_bits: NonZero<u32>) -> usize {
    (value.wrapping_neg() & (1usize << alignment_bits.get()).wrapping_neg()).wrapping_neg()
}

pub const fn align_up_div(value: usize, alignment_bits: NonZero<u32>) -> usize {
    align_up(value, alignment_bits) / (1usize << alignment_bits.get())
}

pub const fn align_down(value: usize, alignment_bits: NonZero<u32>) -> usize {
    (value >> alignment_bits.get()) << alignment_bits.get()
}

pub const fn align_down_div(value: usize, alignment_bits: NonZero<u32>) -> usize {
    value >> alignment_bits.get()
}

#[cfg(test)]
mod tests {
    use core::num::NonZero;

    #[test]
    pub fn align_up() {
        assert_eq!(
            super::align_up(0xFC, unsafe { NonZero::new_unchecked(4) }),
            0x100
        );
    }

    #[test]
    pub fn align_up_div() {
        assert_eq!(
            super::align_up_div(0xFC, unsafe { NonZero::new_unchecked(4) }),
            0x10
        );
    }

    #[test]
    pub fn align_down() {
        assert_eq!(
            super::align_down(0xFC, unsafe { NonZero::new_unchecked(4) }),
            0xF0
        );
    }

    #[test]
    pub fn align_down_div() {
        assert_eq!(
            super::align_down_div(0xFC, unsafe { NonZero::new_unchecked(4) }),
            0xF
        );
    }
}
