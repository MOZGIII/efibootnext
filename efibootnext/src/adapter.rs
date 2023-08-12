//! The platform specific implementation.

use crate::error::{
    EnumerateLoadOptionsError, GetBootNextError, GetLoadOptionError, SetBootNextError,
};
use crate::load_option::LoadOption;
use crate::load_option_iter::LoadOptionIter;
use crate::load_option_number_iter::LoadOptionNumberIter;
use efi_loadopt::EFILoadOpt;
use efivar::{efi::VariableFlags, VarManager};

/// An interface to the OS-specific EFI implementation.
pub struct Adapter {
    /// The actual implementation.
    pub(crate) var_manager: Box<dyn VarManager>,
}

impl Adapter {
    /// Get the load option under the number `num`.
    pub fn get_load_option(&self, num: u16) -> Result<Option<LoadOption>, GetLoadOptionError> {
        let var_name = format_load_option_name(num);
        let full_var_name = efivar::efi::VariableName::new(&var_name);
        let mut buf = make_var_read_buf();
        let (buf, _flags) =
            match map_into_option(read_var(&*self.var_manager, &full_var_name, &mut buf)) {
                Err(err) => return Err(GetLoadOptionError::Efivar(err)),
                Ok(None) => return Ok(None),
                Ok(Some(v)) => v,
            };

        let efiloadopt = EFILoadOpt::decode(buf).map_err(GetLoadOptionError::LoadOptionDecoding)?;
        Ok(Some(LoadOption {
            number: num,
            description: efiloadopt.description,
        }))
    }

    /// Enumerate all the load options using the built-in heuristics.
    pub fn load_options(
        &mut self,
    ) -> Result<
        impl Iterator<Item = Result<LoadOption, GetLoadOptionError>> + '_,
        EnumerateLoadOptionsError,
    > {
        let number_iter = LoadOptionNumberIter::new(&*self.var_manager)
            .map_err(EnumerateLoadOptionsError::Efivar)?;
        Ok(LoadOptionIter::with_number_iter(self, number_iter))
    }

    /// Set the `BootNext` variable value to `num`.
    pub fn set_boot_next(&mut self, num: u16) -> Result<(), SetBootNextError> {
        let full_var_name = efivar::efi::VariableName::new("BootNext");
        self.var_manager
            .write(
                &full_var_name,
                VariableFlags::NON_VOLATILE
                    | VariableFlags::BOOTSERVICE_ACCESS
                    | VariableFlags::RUNTIME_ACCESS,
                &num.to_ne_bytes(),
            )
            .map_err(SetBootNextError::Efivar)?;
        Ok(())
    }

    /// Get the current `BootNext` variable value.
    pub fn get_boot_next(&mut self) -> Result<Option<u16>, GetBootNextError> {
        let full_var_name = efivar::efi::VariableName::new("BootNext");
        let mut buf = make_var_read_buf();
        let result = map_into_option(read_var(&*self.var_manager, &full_var_name, &mut buf));
        let (buf, _flags) = match result.map_err(GetBootNextError::Efivar)? {
            None => return Ok(None),
            Some(val) => val,
        };
        if buf.len() != 2 {
            return Err(GetBootNextError::InvalidValue)?;
        }
        let result = u16::from_ne_bytes([buf[0], buf[1]]);
        Ok(Some(result))
    }

    /// Create [`Self`] with a provided var manager.
    ///
    /// This is an escape hatch for cases where passing the var manager as-is is needed.
    /// However, the var manager is an implementation detail.
    #[cfg(feature = "expose_implementation_details")]
    pub fn from_var_manager(var_manager: Box<dyn efivar::VarManager>) -> Self {
        Self { var_manager }
    }
}

impl Default for Adapter {
    fn default() -> Self {
        Self {
            var_manager: efivar::system(),
        }
    }
}

/// Map the `efivar::Result<T>` in such a way that the value becomes an option and `None` is returned
/// for the "var not found" errors.
fn map_into_option<T>(result: efivar::Result<T>) -> efivar::Result<Option<T>> {
    use efivar::Error;
    match result {
        Ok(v) => Ok(Some(v)),
        Err(Error::VarNotFound { .. }) => Ok(None),
        // The underlying API error handler is too specific, and Rust changed the API by introducing another "unknown"
        // variant at the [`std::io::Error`]. The error we are looking for used to fall under the `Other` category,
        // but now it goes into *undocumented* `Uncategorized`. Thus the check for the `kind == Other` is not erroneus!
        // Core devs really have to be more careful with this.
        #[cfg(windows)]
        Err(Error::VarUnknownError { error, .. }) if error.raw_os_error() == Some(203) => Ok(None),
        Err(err) => Err(err)?,
    }
}

/// Format the number as a load option name.
fn format_load_option_name(num: u16) -> String {
    format!("Boot{:04X}", num)
}

/// A helper for reading an env var into a buffer.
/// Can reuse the buffer for the extra efficiency.
fn read_var<'b>(
    reader: &dyn efivar::VarManager,
    name: &efivar::efi::VariableName,
    buf: &'b mut [u8],
) -> std::result::Result<(&'b [u8], efivar::efi::VariableFlags), efivar::Error> {
    let (n, flags) = reader.read(name, buf)?;
    Ok((&buf[..n], flags))
}

/// Create a buffer for reading EFI variables with an opinionated size.
fn make_var_read_buf() -> Vec<u8> {
    vec![0u8; 20 * 1024] // should be enough
}

#[test]
fn formats_validly() {
    assert_eq!(format_load_option_name(0), "Boot0000");
    assert_eq!(format_load_option_name(1), "Boot0001");
    assert_eq!(format_load_option_name(2), "Boot0002");
    assert_eq!(format_load_option_name(10), "Boot000A");
    assert_eq!(format_load_option_name(16), "Boot0010");
}
