use crate::constants::get_paging_depth;
use core::num::NonZero;

/// Bit shift required to offset page indexes.
#[must_use]
pub const fn page_bits() -> NonZero<u32> {
    // Safety: Value is non-zero.
    unsafe { NonZero::<u32>::new_unchecked(12) }
}

/// Bit shift required to offset large page indexes.
#[must_use]
pub const fn large_page_bits() -> NonZero<u32> {
    // Safety: Value is non-zero.
    unsafe { NonZero::<u32>::new_unchecked(page_bits().get() + table_index_bits().get()) }
}

/// Bit shift required to offset huge page indexes.
#[must_use]
pub const fn huge_page_bits() -> NonZero<u32> {
    // Safety: Value is non-zero.
    unsafe { NonZero::<u32>::new_unchecked(large_page_bits().get() + table_index_bits().get()) }
}

/// The size of a page in bytes.
#[must_use]
pub const fn page_size() -> usize {
    1 << page_bits().get()
}

/// The size of a large page in bytes.
#[must_use]
pub const fn large_page_size() -> usize {
    1 << large_page_bits().get()
}

/// The size of a huge page in bytes.
#[must_use]
pub const fn huge_page_size() -> usize {
    1 << huge_page_bits().get()
}

/// Bit-mask of non-index page bytes.
#[must_use]
pub const fn page_mask() -> usize {
    page_size() - 1
}

/// Bit-mask of non-index large page bytes.
#[must_use]
pub const fn large_page_mask() -> usize {
    large_page_size() - 1
}

/// Bit-mask of non-index huge page bytes.
#[must_use]
pub const fn huge_page_mask() -> usize {
    huge_page_size() - 1
}

/// Shift (in bits) of a page table index.
#[must_use]
pub const fn table_index_bits() -> NonZero<u32> {
    // Safety: Value is non-zero.
    unsafe { NonZero::<u32>::new_unchecked(9) }
}

/// Size (in bytes) of a page table index.
#[must_use]
pub const fn table_index_size() -> usize {
    1 << table_index_bits().get()
}

/// Bit-mask of a page table index.
#[must_use]
pub const fn table_index_mask() -> usize {
    table_index_size() - 1
}

/// Number of bits in a canonical physical address.
#[must_use]
pub const fn physical_address_bits() -> NonZero<u32> {
    // Safety: Value is non-zero.
    unsafe { NonZero::<u32>::new_unchecked(52) }
}

/// The maximum physical address size (in bytes).
#[must_use]
pub const fn physical_address_size() -> usize {
    1 << physical_address_bits().get()
}

/// Bit-mask of canonical physical bits.
#[must_use]
pub const fn physical_address_mask() -> usize {
    physical_address_size() - 1
}

/// Checks if the provided `physical_address` is canonical.
#[must_use]
pub const fn is_physical_address_canonical(physical_address: usize) -> bool {
    (physical_address & !physical_address_mask()) == 0
}

/// Bit-shift to reach non-canonical bits of a virtual address.
#[must_use]
pub fn virtual_address_bits() -> NonZero<u32> {
    let table_indexes_shift = table_index_bits().checked_mul(get_paging_depth()).unwrap();
    let total_shift = table_indexes_shift.checked_add(page_bits().get()).unwrap();

    NonZero::<u32>::new(total_shift.get()).unwrap()
}

/// Checks whether a provided address has only the canonical virtual bits.
#[must_use]
pub fn is_virtual_address_canonical(virtual_address: usize) -> bool {
    let sign_extension_check_shift = virtual_address_bits().get().checked_sub(1).unwrap();

    matches!(virtual_address >> sign_extension_check_shift, 0 | 0x1FFFF)
}

/// Truncates all non-canonical physical bits from an address.
#[must_use]
pub fn truncate_physical_address(address: usize) -> usize {
    address & physical_address_mask()
}

/// Truncates all non-canonical virtual bits from an address.
#[must_use]
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
pub fn truncate_virtual_address(address: usize) -> usize {
    let sign_extension_shift = usize::BITS - virtual_address_bits().get();
    (((address << sign_extension_shift) as isize) >> sign_extension_shift) as usize
}
