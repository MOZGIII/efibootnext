#![warn(rust_2018_idioms)]

mod adapter;
pub mod error;
mod heuristics_load_option_number_iter;
mod load_option;
mod load_option_iter;

pub use adapter::Adapter;
pub use failure::Error;
pub use load_option::LoadOption;

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(feature = "expose_implementation_details")]
pub mod implementation_details {
    //! We do not provide any interface stability guarantees to the
    //! implementation details.

    pub use efivar;

    use crate::Adapter;
    impl Adapter {
        pub fn from_var_manager(var_manager: Box<dyn efivar::VarManager>) -> Self {
            Self { var_manager }
        }
    }
}
