use crate::error::{InvalidBootNextValue, NoSuchLoadOption};
use crate::heuristics_load_option_number_iter::HeuristicsLoadOptionNumberIter;
use crate::load_option::LoadOption;
use crate::load_option_iter::LoadOptionIter;
use crate::Result;
use efi_loadopt::EFILoadOpt;
use efivar::{efi::VariableFlags, VarManager};

pub struct Adapter {
    pub(crate) var_manager: Box<dyn VarManager>,
}

impl Adapter {
    pub fn get_load_option(&mut self, num: u16) -> Result<LoadOption> {
        let var_name = format_load_option_name(num);
        let full_var_name = efivar::efi::to_fullname(&var_name);
        let (_flags, buf) = match map_result(self.var_manager.read(&full_var_name)) {
            Err(err) => return Err(err)?,
            Ok(None) => return Err(NoSuchLoadOption::new(num))?,
            Ok(Some(v)) => v,
        };

        let efiloadopt = EFILoadOpt::decode(&buf)?;
        Ok(LoadOption::new(num, efiloadopt.description.to_owned()))
    }

    pub fn load_options(&mut self) -> impl Iterator<Item = Result<LoadOption>> + '_ {
        let number_iter = HeuristicsLoadOptionNumberIter::new();
        LoadOptionIter::with_number_iter(self, number_iter)
    }

    pub fn set_boot_next(&mut self, num: u16) -> Result<()> {
        let full_var_name = efivar::efi::to_fullname("BootNext");
        let _ = self.var_manager.write(
            &full_var_name,
            VariableFlags::NON_VOLATILE
                | VariableFlags::BOOTSERVICE_ACCESS
                | VariableFlags::RUNTIME_ACCESS,
            &num.to_ne_bytes(),
        )?;
        Ok(())
    }

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

fn map_result<T>(result: efivar::Result<T>) -> Result<Option<T>> {
    use efivar::Error;
    match result {
        Ok(v) => Ok(Some(v)),
        Err(Error::VarNotFound { .. }) => Ok(None),
        Err(err) => Err(err)?,
    }
}

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
