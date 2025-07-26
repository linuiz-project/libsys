#![allow(clippy::missing_panics_doc)]

use core::num::NonZero;

#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(not(test))]
static PAGING_DEPTH: spin::Once<NonZero<u32>> = spin::Once::new();

pub fn get_paging_depth() -> NonZero<u32> {
    cfg_select! {
        // Safety: Value is non-zero.
        test => { unsafe { NonZero::new_unchecked(4) } }
        _ => { *PAGING_DEPTH.get().expect("paging depth has not been set") }
    }
}

/// Sets the current paging depth.
///
/// # Safety
///
/// - `value` must be the current paging depth.
///
/// # Remarks
///
/// If `value` is *not* the current paging depth, it will be possible to
/// construct invalid [`Address`][crate::address::Address] kinds that rely on
/// checking the current paging depth to ensure canonicality.
#[cfg(not(test))]
pub unsafe fn set_paging_depth(paging_depth: NonZero<u32>) {
    PAGING_DEPTH.call_once(|| paging_depth);
}
