#![no_std]

// TODO account for pointer width not matching register width

mod address;
pub use address::*;

mod constants;
pub use constants::*;

pub mod syscall;

#[macro_use]
extern crate static_assertions;

use core::num::NonZeroU32;

pub const fn align_up(value: usize, alignment_bits: NonZeroU32) -> usize {
    (value.wrapping_neg() & (1usize << alignment_bits.get()).wrapping_neg()).wrapping_neg()
}

pub const fn align_up_div(value: usize, alignment_bits: NonZeroU32) -> usize {
    align_up(value, alignment_bits) / (1usize << alignment_bits.get())
}

pub const fn align_down(value: usize, alignment_bits: NonZeroU32) -> usize {
    value & !((1usize << alignment_bits.get()) - 1)
}

pub const fn align_down_div(value: usize, alignment_bits: NonZeroU32) -> usize {
    align_down(value, alignment_bits) / (1usize << alignment_bits.get())
}
