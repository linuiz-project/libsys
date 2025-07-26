use crate::{
    address::{Address, AddressKind, NonCanonicalError, Virtual},
    constants::{is_virtual_address_canonical, page_mask, page_shift, truncate_virtual_address},
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
    pub fn new(address: usize) -> Result<Self, NonCanonicalError> {
        if ((address & page_mask()) == 0) && is_virtual_address_canonical(address) {
            Ok(Self(address))
        } else {
            Err(NonCanonicalError)
        }
    }

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

    #[must_use]
    pub fn get(&self) -> Address<Virtual> {
        // Safety: `Address<Page>` is a superset of `Address<Virtual>`s validition ruleset.
        unsafe { Address::<Virtual>::new_unsafe(self.0) }
    }

    pub fn from_index(index: usize) -> Result<Self, NonCanonicalError> {
        let virtual_address = index << page_shift().get();

        if is_virtual_address_canonical(virtual_address) {
            Ok(Self(virtual_address))
        } else {
            Err(NonCanonicalError)
        }
    }

    #[must_use]
    pub fn index(&self) -> usize {
        self.0 >> page_shift().get()
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

impl<T> From<*mut T> for Address<Page> {
    fn from(value: *mut T) -> Self {
        Self(value.addr())
    }
}
