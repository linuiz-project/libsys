#[derive(Debug)]
pub struct Physical;

impl super::Addressable for Physical {
    type Init = usize;
    type Repr = usize;
    type Get = usize;

    const DEBUG_NAME: &'static str = "Address<Physical>";

    fn new(init: Self::Init) -> Option<Self::Repr> {
        crate::constants::is_physical_address_canonical(init).then_some(init)
    }

    fn new_truncate(init: Self::Init) -> Self::Repr {
        init & crate::constants::physical_address_mask()
    }

    unsafe fn new_unsafe(init: Self::Init) -> Self::Repr {
        init
    }

    fn get(repr: Self::Repr) -> Self::Get {
        repr
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
