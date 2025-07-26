use crate::{
    Address, Addressable, NonCanonicalError, Physical, is_physical_address_canonical, page_mask,
    page_shift, physical_address_mask,
};

pub struct Frame;

impl Addressable for Frame {
    type Init = usize;
    type Repr = usize;
    type Get = Address<Physical>;

    const DEBUG_NAME: &'static str = "Address<Frame>";

    fn new(init: Self::Init) -> Option<Self::Repr> {
        (((init & page_mask()) == 0) && is_physical_address_canonical(init)).then_some(init)
    }

    fn new_truncate(init: Self::Init) -> Self::Repr {
        init & physical_address_mask() & !page_mask()
    }

    unsafe fn new_unsafe(init: Self::Init) -> Self::Repr {
        init
    }

    fn get(repr: Self::Repr) -> Self::Get {
        Address::new_truncate(repr)
    }
}

impl Address<Frame> {
    pub fn from_index(index: usize) -> Result<Self, NonCanonicalError> {
        let physical_address = index << page_shift().get();

        if is_physical_address_canonical(physical_address) {
            Ok(Self(physical_address))
        } else {
            Err(NonCanonicalError)
        }
    }

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
