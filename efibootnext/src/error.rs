//! Errors.

/// An alias for the export.
#[cfg(feature = "expose_implementation_details")]
pub use efivar::Error as EfivarError;

/// An alias for the export.
#[cfg(feature = "expose_implementation_details")]
pub use efi_loadopt::DecodeError as LoadOptionDecodingError;

/// An error that can occur when reading load option.
#[derive(Debug, thiserror::Error)]
pub enum EnumerateLoadOptionsError {
    /// Something went wrong at [`efivar`] level.
    #[error("low-level error: {0}")]
    Efivar(efivar::Error),
}

/// An error that can occur when reading load option.
#[derive(Debug, thiserror::Error)]
pub enum GetLoadOptionError {
    /// Something went wrong at [`efivar`] level.
    #[error("low-level error: {0}")]
    Efivar(efivar::Error),
    /// The `LoadOption` decoding has failed.
    #[error("load option decoding error: {0}")]
    LoadOptionDecoding(efi_loadopt::DecodeError),
}

/// An error that can occur when setting the boot next value.
#[derive(Debug, thiserror::Error)]
pub enum SetBootNextError {
    /// Something went wrong at [`efivar`] level.
    #[error("low-level error: {0}")]
    Efivar(efivar::Error),
}

/// An error that can occur when setting the boot next value.
#[derive(Debug, thiserror::Error)]
pub enum GetBootNextError {
    /// Something went wrong at [`efivar`] level.
    #[error("low-level error: {0}")]
    Efivar(efivar::Error),
    /// The underlying value read is invalid.
    #[error("boot next value is not valid")]
    InvalidValue,
}
