use crate::{
    Address, Physical, is_physical_address_canonical, page_mask, page_shift, physical_address_mask,
};

pub struct Frame;

impl super::Addressable for Frame {
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

    fn get(repr: Self::Repr) -> Self::Get {
        Address::new_truncate(repr)
    }
}

impl super::IndexAddressable for Frame {
    fn from_index(index: usize) -> Option<Self::Repr> {
        let physical_address = index << page_shift().get();
        is_physical_address_canonical(physical_address).then_some(physical_address)
    }

    fn index(repr: Self::Repr) -> usize {
        repr >> page_shift().get()
    }
}
