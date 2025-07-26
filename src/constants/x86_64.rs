use core::num::NonZero;

/// Bit shift required to offset page indexes.
#[must_use]
pub const fn page_shift() -> NonZero<u32> {
    NonZero::<u32>::new(12).unwrap()
}

/// Bit shift required to offset mega page indexes.
#[must_use]
pub const fn mega_page_shift() -> NonZero<u32> {
    page_shift().checked_add(table_index_shift().get()).unwrap()
}

/// Bit shift required to offset giga page indexes.
#[must_use]
pub const fn giga_page_shift() -> NonZero<u32> {
    mega_page_shift()
        .checked_add(table_index_shift().get())
        .unwrap()
}

/// The size of a page in bytes.
#[must_use]
pub const fn page_size() -> usize {
    1usize.checked_shl(page_shift().get()).unwrap()
}

/// The size of a mega page in bytes.
#[must_use]
pub const fn mega_page_size() -> usize {
    1usize.checked_shl(mega_page_shift().get()).unwrap()
}

/// The size of a giga page in bytes.
#[must_use]
pub const fn giga_page_size() -> usize {
    1usize.checked_shl(giga_page_shift().get()).unwrap()
}

/// Bit-mask of non-index page bytes.
#[must_use]
pub const fn page_mask() -> usize {
    page_size().checked_sub(1).unwrap()
}

/// Bit-mask of non-index mega page bytes.
#[must_use]
pub const fn mega_page_mask() -> usize {
    mega_page_size().checked_sub(1).unwrap()
}

/// Bit-mask of non-index giga page bytes.
#[must_use]
pub const fn giga_page_mask() -> usize {
    giga_page_size().checked_sub(1).unwrap()
}

/// Shift (in bits) of a page table index.
#[must_use]
pub const fn table_index_shift() -> NonZero<u32> {
    NonZero::<u32>::new(9).unwrap()
}

/// Size (in bytes) of a page table index.
#[must_use]
pub const fn table_index_size() -> usize {
    1 << table_index_shift().get()
}

/// Bit-mask of a page table index.
#[must_use]
pub const fn table_index_mask() -> usize {
    table_index_size().checked_sub(1).unwrap()
}

/// Bit-mask of canonical physical bits.
#[must_use]
pub const fn physical_address_mask() -> usize {
    0x000F_FFFF_FFFF_FFFF
}
/// Checks if the provided `physical_address` is canonical.
#[must_use]
pub const fn is_physical_address_canonical(physical_address: usize) -> bool {
    (physical_address & !physical_address_mask()) == 0
}

/// The maximum paging depth (either 4 or 5) of the current environment.
#[must_use]
pub fn max_paging_depth() -> u32 {
    const CR4_LA57_BIT: usize = 1 << 12;

    let cr4: usize;
    unsafe {
        core::arch::asm!(
            "mov {}, cr4",
            out(reg) cr4,
            options(nomem, pure)
        );
    }

    if (cr4 & CR4_LA57_BIT) == 0 { 4 } else { 5 }
}

/// Bit-shift to reach non-canonical bits of a virtual address.
#[must_use]
pub fn virtual_address_noncanonical_shift() -> NonZero<u32> {
    let table_indexes_shift = table_index_shift()
        .get()
        .checked_mul(max_paging_depth())
        .unwrap();
    let total_shift = table_indexes_shift.checked_add(page_shift().get()).unwrap();

    NonZero::<u32>::new(total_shift).unwrap()
}

/// Checks whether a provided address has only the canonical virtual bits.
#[must_use]
pub fn is_virtual_address_canonical(virtual_address: usize) -> bool {
    let sign_extension_check_shift = virtual_address_noncanonical_shift()
        .get()
        .checked_sub(1)
        .unwrap();

    matches!(virtual_address >> sign_extension_check_shift, 0 | 0x1ffff)
}

#[must_use]
pub fn truncate_physical_address(address: usize) -> usize {
    address & physical_address_mask()
}

#[must_use]
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
pub fn truncate_virtual_address(address: usize) -> usize {
    let sign_extension_shift = usize::BITS - virtual_address_noncanonical_shift().get();
    (((address << sign_extension_shift) as isize) >> sign_extension_shift) as usize
}
