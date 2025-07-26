use crate::{
    Address, Addressable, is_virtual_address_canonical, virtual_address_noncanonical_shift,
};

#[derive(Debug)]
pub struct Virtual;

impl Addressable for Virtual {
    type Init = usize;
    type Repr = usize;
    type Get = usize;

    const DEBUG_NAME: &'static str = "Address<Virtual>";

    fn new(init: Self::Init) -> Option<Self::Repr> {
        is_virtual_address_canonical(init).then_some(init)
    }

    fn new_truncate(init: Self::Init) -> Self::Repr {
        let sign_extension_shift = Self::Init::BITS - virtual_address_noncanonical_shift().get();
        (((init << sign_extension_shift) as isize) >> sign_extension_shift) as Self::Repr
    }

    unsafe fn new_unsafe(init: Self::Init) -> Self::Repr {
        init
    }

    fn get(repr: Self::Repr) -> Self::Get {
        repr
    }
}

impl<T> From<*mut T> for Address<Virtual> {
    fn from(value: *mut T) -> Self {
        Self(value.addr())
    }
}
