use crate::{
    address::{Address, AddressKind, NonCanonicalError},
    constants::{is_physical_address_canonical, truncate_physical_address},
};

#[derive(Debug)]
pub struct Physical;

impl AddressKind for Physical {
    type Repr = usize;
}

impl Copy for Address<Physical> {}
impl Clone for Address<Physical> {
    fn clone(&self) -> Self {
        *self
    }
}

impl Eq for Address<Physical> {}
impl PartialEq for Address<Physical> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for Address<Physical> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Address<Physical> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Address<Physical> {
    pub fn new(address: usize) -> Result<Self, NonCanonicalError> {
        if is_physical_address_canonical(address) {
            Ok(Self(address))
        } else {
            Err(NonCanonicalError)
        }
    }

    #[must_use]
    pub fn new_truncate(address: usize) -> Self {
        Self(truncate_physical_address(address))
    }

    /// # Safety
    ///
    /// - `address` must have only canonical physical address bits set.
    #[must_use]
    pub unsafe fn new_unsafe(address: usize) -> Self {
        Self(address)
    }

    #[must_use]
    pub fn get(&self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{Address, Physical};

    #[test]
    fn get() {
        assert_eq!((unsafe { Address::<Physical>::new_unsafe(0xF) }).get(), 0xF);
    }

    #[test]
    fn new() {
        assert_eq!(Address::<Physical>::new(0xFFF0_0000_0000_000F), None);
        assert_eq!(Address::<Physical>::new(0xF).map(Address::get), Some(0xF));
    }

    #[test]
    fn new_truncate() {
        assert_eq!(
            Address::<Physical>::new_truncate(0xFFF0_0000_0000_000F).get(),
            0xF
        );
    }
}

impl core::fmt::Debug for Address<Physical> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Address<Physical>").field(&self.0).finish()
    }
}
