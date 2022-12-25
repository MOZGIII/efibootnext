//! Errors.

use failure_derive::Fail;

/// An error that is returned when the requested load option was not found.
#[derive(Debug, Fail)]
#[fail(display = "load option with number {:04X} does not exist", number)]
pub struct NoSuchLoadOption {
    /// Load option number.
    pub number: u16,
}

/// An error that is returned when setting the `BootNext` to an invalid value.
#[derive(Debug, Fail)]
#[fail(display = "boot next value is not valid")]
pub struct InvalidBootNextValue;
