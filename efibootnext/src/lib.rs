//! Manage EFI `BootNext` variable and inspect available boot options.

mod adapter;
pub mod error;
mod load_option;
mod load_option_iter;
mod load_option_number_iter;

pub use adapter::Adapter;
pub use load_option::LoadOption;

#[cfg(feature = "expose_implementation_details")]
pub use efivar;

#[cfg(feature = "expose_implementation_details")]
pub use efi_loadopt;
