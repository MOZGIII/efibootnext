//! The platform specific implementation.

use crate::error::{InvalidBootNextValue, NoSuchLoadOption};
use crate::heuristics_load_option_number_iter::HeuristicsLoadOptionNumberIter;
use crate::load_option::LoadOption;
use crate::load_option_iter::LoadOptionIter;
use crate::Result;
use efi_loadopt::EFILoadOpt;
use efivar::{efi::VariableFlags, VarManager};

/// An interface to the OS-specific EFI implementation.
pub struct Adapter {
    /// The actual implementation.
    pub(crate) var_manager: Box<dyn VarManager>,
}

impl Adapter {
    /// Get the load option under the number `num`.
    pub fn get_load_option(&mut self, num: u16) -> Result<LoadOption> {
        let var_name = format_load_option_name(num);
        let full_var_name = efivar::efi::to_fullname(&var_name);
        let (_flags, buf) = match map_result(self.var_manager.read(&full_var_name)) {
            Err(err) => return Err(err)?,
            Ok(None) => return Err(NoSuchLoadOption { number: num })?,
            Ok(Some(v)) => v,
        };

        let efiloadopt = EFILoadOpt::decode(&buf)?;
        Ok(LoadOption {
            number: num,
            description: efiloadopt.description,
        })
    }

    /// Enumerate all the load options using the built-in heuristics.
    pub fn load_options(&mut self) -> impl Iterator<Item = Result<LoadOption>> + '_ {
        let number_iter = HeuristicsLoadOptionNumberIter::new();
        LoadOptionIter::with_number_iter(self, number_iter)
    }

    /// Set the `BootNext` variable value to `num`.
    pub fn set_boot_next(&mut self, num: u16) -> Result<()> {
        let full_var_name = efivar::efi::to_fullname("BootNext");
        self.var_manager.write(
            &full_var_name,
            VariableFlags::NON_VOLATILE
                | VariableFlags::BOOTSERVICE_ACCESS
                | VariableFlags::RUNTIME_ACCESS,
            &num.to_ne_bytes(),
        )?;
        Ok(())
    }

    /// Get the current `BootNext` variable value.
    pub fn get_boot_next(&mut self) -> Result<Option<u16>> {
        let full_var_name = efivar::efi::to_fullname("BootNext");
        let (_flags, buf) = match map_result(self.var_manager.read(&full_var_name))? {
            None => return Ok(None),
            Some(val) => val,
        };
        if buf.len() != 2 {
            return Err(InvalidBootNextValue)?;
        }
        let result = u16::from_ne_bytes([buf[0], buf[1]]);
        Ok(Some(result))
    }
}

impl Default for Adapter {
    fn default() -> Self {
        Self {
            var_manager: efivar::system(),
        }
    }
}

/// Map the EFI result into the crate error.
fn map_result<T>(result: efivar::Result<T>) -> Result<Option<T>> {
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

#[test]
fn formats_validly() {
    assert_eq!(format_load_option_name(0), "Boot0000");
    assert_eq!(format_load_option_name(1), "Boot0001");
    assert_eq!(format_load_option_name(2), "Boot0002");
    assert_eq!(format_load_option_name(10), "Boot000A");
    assert_eq!(format_load_option_name(16), "Boot0010");
}
