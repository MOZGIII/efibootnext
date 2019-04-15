use crate::LoadOption;
use crate::NoSuchLoadOption;
use efi_loadopt::EFILoadOpt;
use failure::Error;

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

pub fn load_option_by_num(
    var_manager: &mut efivar::VarManager,
    num: u16,
) -> Result<LoadOption, Error> {
    let var_name = format_load_option_name(num);
    let full_var_name = efivar::efi::to_fullname(&var_name);
    let (_flags, value) = match var_manager.read(&full_var_name) {
        Ok(v) => v,
        Err(err) => {
            if is_no_such_load_option_error(&err) {
                return Err(NoSuchLoadOption::new(num).into());
            }
            return Err(err.into());
        }
    };

    let efiloadopt = EFILoadOpt::decode(&value)?;
    Ok(LoadOption::new(num, efiloadopt.description.to_owned()))
}

#[cfg(not(target_os = "windows"))]
fn is_no_such_load_option_error(err: &std::io::Error) -> bool {
    err.kind() == std::io::ErrorKind::NotFound
}

#[cfg(target_os = "windows")]
fn is_no_such_load_option_error(err: &std::io::Error) -> bool {
    err.kind() == std::io::ErrorKind::Other && err.raw_os_error() == Some(203)
}

pub fn set_boot_next(var_manager: &mut efivar::VarManager, num: u16) -> Result<(), Error> {
    let full_var_name = efivar::efi::to_fullname("BootNext");
    use efivar::efi::VariableFlags;
    let _ = var_manager.write(
        &full_var_name,
        VariableFlags::NON_VOLATILE
            | VariableFlags::BOOTSERVICE_ACCESS
            | VariableFlags::RUNTIME_ACCESS,
        &num.to_ne_bytes(),
    )?;
    Ok(())
}
