use crate::{
    address::{Address, AddressKind, NonCanonicalError, Virtual},
    constants::{
        is_virtual_address_canonical, page_mask, page_bits, page_size, truncate_virtual_address,
    },
};

pub struct Page;

impl AddressKind for Page {
    type Repr = usize;
}

impl Copy for Address<Page> {}
impl Clone for Address<Page> {
    fn clone(&self) -> Self {
        *self
    }
}

impl Eq for Address<Page> {}
impl PartialEq for Address<Page> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for Address<Page> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Address<Page> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Address<Page> {
    /// Creates a new [`Address<Page>`] with the provided address.
    ///
    /// # Errors
    ///
    /// - [`NonCanonicalError`] if `address` contains any non-canonical bits.
    pub fn new(address: usize) -> Result<Self, NonCanonicalError> {
        if ((address & page_mask()) == 0) && is_virtual_address_canonical(address) {
            Ok(Self(address))
        } else {
            Err(NonCanonicalError)
        }
    }

    /// Creates a new [`Address<Page>`] with the provided address, truncating
    /// any non-canonical bits.
    #[must_use]
    pub fn new_truncate(address: usize) -> Self {
        Self(truncate_virtual_address(address) & !page_mask())
    }

    /// # Safety
    ///
    /// - `address` must be page-aligned.
    /// - `address` must have only canonical virtual address bits set.
    #[must_use]
    pub unsafe fn new_unsafe(address: usize) -> Self {
        Self(address)
    }

    /// Gets the inner value.
    #[must_use]
    pub fn get(&self) -> Address<Virtual> {
        // Safety: `Address<Page>` is a superset of `Address<Virtual>`s canonicality.
        unsafe { Address::<Virtual>::new_unsafe(self.0) }
    }

    /// Creates a new [`Address<Page>`] with the provided frame index.
    ///
    /// # Errors
    ///
    /// - [`NonCanonicalError`] if `index` would create a non-canonical address.
    pub fn from_index(index: usize) -> Result<Self, NonCanonicalError> {
        let virtual_address = index << page_bits().get();

        if is_virtual_address_canonical(virtual_address) {
            Ok(Self(virtual_address))
        } else {
            Err(NonCanonicalError)
        }
    }

    /// Gets the index of the page this address points to.
    #[must_use]
    pub fn index(&self) -> usize {
        self.0 >> page_bits().get()
    }
}

impl core::iter::Step for Address<Page> {
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
        core::iter::Step::steps_between(&start.index(), &end.index())
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        start
            .index()
            .checked_add(count)
            .and_then(|next_index| Self::from_index(next_index).ok())
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        start
            .index()
            .checked_sub(count)
            .and_then(|next_index| Self::from_index(next_index).ok())
    }
}

impl<T> TryFrom<*mut T> for Address<Page> {
    type Error = NonCanonicalError;

    fn try_from(value: *mut T) -> Result<Self, Self::Error> {
        if value.is_aligned_to(page_size()) {
            Ok(Self(value.addr()))
        } else {
            Err(NonCanonicalError)
        }
    }
}

impl core::fmt::Debug for Address<Page> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Address<Page>").field(&self.0).finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::address::{Address, NonCanonicalError, Page, Virtual};

    #[test]
    fn get() {
        assert_eq!(
            (unsafe { Address::<Page>::new_unsafe(0xF000) }).get(),
            unsafe { Address::<Virtual>::new_unsafe(0xF000) }
        );
    }

    #[test]
    fn new() {
        assert_eq!(
            Address::<Page>::new(0xFFFF_0000_0000_F000),
            Err(NonCanonicalError)
        );
    }

    #[test]
    fn new_truncate() {
        assert_eq!(
            Address::<Page>::new_truncate(0xFFF0_0000_0000_F000).get(),
            Address::<Virtual>::new_truncate(0xFFF0_0000_0000_F000)
        );
    }

    #[test]
    fn index() {
        assert_eq!(
            Address::<Page>::from_index(0xF).map(|page| page.index()),
            Ok(0xF)
        );
    }

    #[test]
    fn from_index() {
        assert_eq!(
            Address::<Page>::from_index(0xFFF0_0000_0000_F),
            Err(NonCanonicalError)
        );
        assert_eq!(
            Address::<Page>::from_index(0xF).map(|address| address.get()),
            Ok(unsafe { Address::<Virtual>::new_unsafe(0xF000) })
        );
    }
}
