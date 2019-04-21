use crate::error::{InvalidBootNextValue, NoSuchLoadOption};
use crate::LoadOption;
use crate::Result;
use efi_loadopt::EFILoadOpt;
use efivar::efi::VariableFlags;

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

fn get_var(
    var_manager: &mut efivar::VarManager,
    name: &str,
) -> Result<Option<(VariableFlags, Vec<u8>)>> {
    use efivar::Error;
    match var_manager.read(name) {
        Ok(v) => Ok(Some(v)),
        Err(Error::VarNotFound { .. }) => Ok(None),
        Err(err) => Err(err)?,
    }
}

pub fn get_load_option(var_manager: &mut efivar::VarManager, num: u16) -> Result<LoadOption> {
    let var_name = format_load_option_name(num);
    let full_var_name = efivar::efi::to_fullname(&var_name);
    let (_flags, value) = match get_var(var_manager, &full_var_name) {
        Err(err) => return Err(err)?,
        Ok(None) => return Err(NoSuchLoadOption::new(num))?,
        Ok(Some(v)) => v,
    };

    let efiloadopt = EFILoadOpt::decode(&value)?;
    Ok(LoadOption::new(num, efiloadopt.description.to_owned()))
}

pub fn set_boot_next(var_manager: &mut efivar::VarManager, num: u16) -> Result<()> {
    let full_var_name = efivar::efi::to_fullname("BootNext");
    let _ = var_manager.write(
        &full_var_name,
        VariableFlags::NON_VOLATILE
            | VariableFlags::BOOTSERVICE_ACCESS
            | VariableFlags::RUNTIME_ACCESS,
        &num.to_ne_bytes(),
    )?;
    Ok(())
}

pub fn get_boot_next(var_manager: &mut efivar::VarManager) -> Result<Option<u16>> {
    let full_var_name = efivar::efi::to_fullname("BootNext");
    let (_flags, buf) = match get_var(var_manager, &full_var_name)? {
        None => return Ok(None),
        Some(val) => val,
    };
    if buf.len() != 2 {
        return Err(InvalidBootNextValue)?;
    }
    let result = u16::from_ne_bytes([buf[0], buf[1]]);
    Ok(Some(result))
}
