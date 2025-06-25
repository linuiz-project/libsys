use core::num::NonZeroU32;

/// Bit shift required to offset page indexes.
pub const fn page_shift() -> NonZeroU32 {
    NonZeroU32::new(12).unwrap()
}

/// Bit shift required to offset mega page indexes.
pub const fn mega_page_shift() -> NonZeroU32 {
    page_shift().checked_add(table_index_shift().get()).unwrap()
}

/// Bit shift required to offset giga page indexes.
pub const fn giga_page_shift() -> NonZeroU32 {
    mega_page_shift()
        .checked_add(table_index_shift().get())
        .unwrap()
}

/// The size of a page in bytes.
pub const fn page_size() -> usize {
    1 << page_shift().get()
}

/// The size of a mega page in bytes.
pub const fn mega_page_size() -> usize {
    512 * page_size()
}

/// The size of a giga page in bytes.
pub const fn giga_page_size() -> usize {
    512 * mega_page_size()
}

/// Bit-mask of non-index page bytes.
pub const fn page_mask() -> usize {
    page_size().checked_sub(1).unwrap()
}

/// Bit-mask of non-index mega page bytes.
pub const fn mega_page_mask() -> usize {
    mega_page_size().checked_sub(1).unwrap()
}

/// Bit-mask of non-index giga page bytes.
pub const fn giga_page_mask() -> usize {
    giga_page_size().checked_sub(1).unwrap()
}

/// Shift (in bits) of a page table index.
pub const fn table_index_shift() -> NonZeroU32 {
    NonZeroU32::new(9).unwrap()
}

/// Size (in bytes) of a page table index.
pub const fn table_index_size() -> usize {
    1 << table_index_shift().get()
}

/// Bit-mask of a page table index.
pub const fn table_index_mask() -> usize {
    table_index_size().checked_sub(1).unwrap()
}

/// Bit-mask of canonical physical bits.
pub const fn physical_address_mask() -> usize {
    0x000F_FFFF_FFFF_FFFF
}
/// Checks if the provided `physical_address` is canonical.
pub const fn is_physical_address_canonical(physical_address: usize) -> bool {
    (physical_address & !physical_address_mask()) == 0
}

/// The maximum paging depth (either 4 or 5) of the current environment.
pub fn max_paging_depth() -> u32 {
    const CR4_LA57_BIT: u64 = 1 << 12;

    let cr4: u64;

    unsafe {
        core::arch::asm!(
            "mov {}, cr4",
            out(reg) cr4,
            options(nomem, pure)
        )
    };

    if (cr4 & CR4_LA57_BIT) == 0 { 4 } else { 5 }
}

/// Bit-shift to reach non-canonical bits of a virtual address.
pub fn virt_noncanonical_shift() -> NonZeroU32 {
    let table_indexes_shift = table_index_shift().get() * max_paging_depth();
    let total_shift = table_indexes_shift + page_shift().get();

    NonZeroU32::new(total_shift).unwrap()
}

/// Checks whether a provided address has only the canonical virtual bits.
pub fn checked_virt_canonical(address: usize) -> bool {
    let sign_extension_check_shift = virt_noncanonical_shift().get().checked_sub(1).unwrap();
    matches!(address >> sign_extension_check_shift, 0 | 0x1ffff)
}
