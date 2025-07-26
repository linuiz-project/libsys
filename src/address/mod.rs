mod frame;
pub use frame::*;

mod page;
pub use page::*;

mod physical;
pub use physical::*;

mod r#virtual;
pub use r#virtual::*;

mod error;
pub use error::*;

pub trait Addressable {
    type Repr;
    type Init;
    type Get;

    const DEBUG_NAME: &'static str;

    fn new(init: Self::Init) -> Option<Self::Repr>;
    fn new_truncate(init: Self::Init) -> Self::Repr;
    unsafe fn new_unsafe(init: Self::Init) -> Self::Repr;

    fn get(repr: Self::Repr) -> Self::Get;
}

#[repr(transparent)]
pub struct Address<Kind: Addressable>(Kind::Repr);

impl<Kind: Addressable> Address<Kind> {
    pub fn new(init: Kind::Init) -> Option<Self> {
        Kind::new(init).map(Self)
    }

    pub fn new_truncate(init: Kind::Init) -> Self {
        Self(Kind::new_truncate(init))
    }

    pub unsafe fn new_unsafe(init: Kind::Init) -> Self {
        // Safety: Caller is required to maintain safety invariants.
        let repr = unsafe { Kind::new_unsafe(init) };
        Self(repr)
    }

    pub fn get(self) -> Kind::Get {
        Kind::get(self.0)
    }
}

impl<I, Repr: Clone, Kind: Addressable<Init = I, Repr = Repr>> Clone for Address<Kind> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<I, Repr: Copy, Kind: Addressable<Init = I, Repr = Repr>> Copy for Address<Kind> {}

impl<I, Repr: PartialEq, Kind: Addressable<Init = I, Repr = Repr>> PartialEq for Address<Kind> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<I, Repr: Eq, Kind: Addressable<Init = I, Repr = Repr>> Eq for Address<Kind> {}

impl<I, Repr: Ord, Kind: Addressable<Init = I, Repr = Repr>> Ord for Address<Kind> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<I, Repr: PartialOrd, Kind: Addressable<Init = I, Repr = Repr>> PartialOrd for Address<Kind> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<I, Repr: core::fmt::Debug, Kind: Addressable<Init = I, Repr = Repr>> core::fmt::Debug
    for Address<Kind>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple(Kind::DEBUG_NAME).field(&self.0).finish()
    }
}
