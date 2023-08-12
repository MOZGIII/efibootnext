//! Manage EFI `BootNext` variable and inspect available boot options.

mod adapter;
mod efivar_load_option_number_iter;
pub mod error;
mod heuristics_load_option_number_iter;
mod load_option;
mod load_option_iter;

pub use adapter::Adapter;
pub use load_option::LoadOption;

#[cfg(feature = "expose_implementation_details")]
pub use efivar;

#[cfg(feature = "expose_implementation_details")]
pub use efi_loadopt;
