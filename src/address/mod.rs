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

#[repr(transparent)]
pub struct Address<T: AddressKind>(T::Repr);

pub trait AddressKind {
    type Repr;
}
