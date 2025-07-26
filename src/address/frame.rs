use crate::{
    address::{Address, AddressKind, NonCanonicalError, Physical},
    constants::{is_physical_address_canonical, page_mask, page_shift, truncate_physical_address},
};

pub struct Frame;

impl AddressKind for Frame {
    type Repr = usize;
}

impl Copy for Address<Frame> {}
impl Clone for Address<Frame> {
    fn clone(&self) -> Self {
        *self
    }
}

impl Eq for Address<Frame> {}
impl PartialEq for Address<Frame> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for Address<Frame> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Address<Frame> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Address<Frame> {
    pub fn new(address: usize) -> Result<Self, NonCanonicalError> {
        if ((address & page_mask()) == 0) && is_physical_address_canonical(address) {
            Ok(Self(address))
        } else {
            Err(NonCanonicalError)
        }
    }

    #[must_use]
    pub fn new_truncate(address: usize) -> Self {
        Self(truncate_physical_address(address) & !page_mask())
    }

    /// # Safety
    ///
    /// - `address` must be page-aligned.
    /// - `address` must have only canonical physical address bits set.
    #[must_use]
    pub unsafe fn new_unsafe(address: usize) -> Self {
        Self(address)
    }

    #[must_use]
    pub fn get(&self) -> Address<Physical> {
        // Safety: `Address<Frame>` is a superset of `Address<Physical>`s validition ruleset.
        unsafe { Address::<Physical>::new_unsafe(self.0) }
    }

    pub fn from_index(index: usize) -> Result<Self, NonCanonicalError> {
        let physical_address = index << page_shift().get();

        if is_physical_address_canonical(physical_address) {
            Ok(Self(physical_address))
        } else {
            Err(NonCanonicalError)
        }
    }

    #[must_use]
    pub fn index(&self) -> usize {
        self.0 >> page_shift().get()
    }
}

impl core::iter::Step for Address<Frame> {
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

impl core::fmt::Debug for Address<Frame> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Address<Frame>").field(&self.0).finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Address, Frame, Physical};

    #[test]
    fn get() {
        assert_eq!(
            (unsafe { Address::<Frame>::new_unsafe(0xF000) }).get(),
            unsafe { Address::<Physical>::new_unsafe(0xF000) }
        );
    }

    #[test]
    fn new() {
        assert_eq!(Address::<Frame>::new(0xFFFF_0000_0000_F000), None);
    }

    #[test]
    fn new_truncate() {
        assert_eq!(
            Address::<Frame>::new_truncate(0xFFF0_0000_0000_F000).get(),
            Address::<Physical>::new_truncate(0xFFF0_0000_0000_F000)
        );
    }

    #[test]
    fn index() {
        assert_eq!(
            Address::<Frame>::from_index(0xF).map(|frame| frame.index()),
            Ok(0xF)
        );
    }

    #[test]
    fn from_index() {
        assert_eq!(
            Address::<Frame>::from_index(0xFFF0_0000_0000_F),
            Err(crate::NonCanonicalError)
        );
        assert_eq!(
            Address::<Frame>::from_index(0xF).map(Address::get),
            Ok(unsafe { Address::<Physical>::new_unsafe(0xF000) })
        );
    }
}
