use crate::{
    Address, Addressable, NonCanonicalError, Virtual, is_virtual_address_canonical, page_mask,
    page_shift,
};

pub struct Page;

impl Addressable for Page {
    type Init = usize;
    type Repr = usize;
    type Get = Address<Virtual>;

    const DEBUG_NAME: &'static str = "Address<Page>";

    fn new(init: Self::Init) -> Option<Self::Repr> {
        (((init & page_mask()) == 0) && is_virtual_address_canonical(init)).then_some(init)
    }

    fn new_truncate(init: Self::Init) -> Self::Repr {
        init & !page_mask()
    }

    unsafe fn new_unsafe(init: Self::Init) -> Self::Repr {
        init
    }

    fn get(repr: Self::Repr) -> Self::Get {
        Address::new_truncate(repr)
    }
}

impl Address<Page> {
    pub fn from_index(index: usize) -> Result<Self, NonCanonicalError> {
        let virtual_address = index << page_shift().get();

        if is_virtual_address_canonical(virtual_address) {
            Ok(Self(virtual_address))
        } else {
            Err(NonCanonicalError)
        }
    }

    pub fn index(&self) -> usize {
        self.0 >> page_shift().get()
    }
}

impl<T> From<*mut T> for Address<Page> {
    fn from(value: *mut T) -> Self {
        Self(value.addr())
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
