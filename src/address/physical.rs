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

    fn get(repr: Self::Repr) -> Self::Get {
        repr
    }
}
