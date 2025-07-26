use crate::{
    address::{Address, AddressKind, NonCanonicalError},
    constants::{is_virtual_address_canonical, truncate_virtual_address},
};

#[derive(Debug)]
pub struct Virtual;

impl AddressKind for Virtual {
    type Repr = usize;
}

impl Copy for Address<Virtual> {}
impl Clone for Address<Virtual> {
    fn clone(&self) -> Self {
        *self
    }
}

impl Eq for Address<Virtual> {}
impl PartialEq for Address<Virtual> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for Address<Virtual> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Address<Virtual> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Address<Virtual> {
    pub fn new(address: usize) -> Result<Self, NonCanonicalError> {
        if is_virtual_address_canonical(address) {
            Ok(Self(address))
        } else {
            Err(NonCanonicalError)
        }
    }

    #[must_use]
    pub fn new_truncate(address: usize) -> Self {
        Self(truncate_virtual_address(address))
    }

    /// # Safety
    ///
    /// - `address` must have only canonical virtual address bits set.
    #[must_use]
    pub unsafe fn new_unsafe(address: usize) -> Self {
        Self(address)
    }

    #[must_use]
    pub fn get(&self) -> usize {
        self.0
    }
}

impl<T> From<*mut T> for Address<Virtual> {
    fn from(ptr: *mut T) -> Self {
        Self(ptr.addr())
    }
}

impl core::fmt::Debug for Address<Virtual> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Address<Virtual>").field(&self.0).finish()
    }
}
