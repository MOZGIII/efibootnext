#[macro_use]
extern crate failure_derive;

mod error;
mod load_option;
mod load_option_iter;
mod operation;

use error::NoSuchLoadOption;

pub use load_option::LoadOption;
pub use load_option_iter::LoadOptionIter;

pub fn load_options(var_manager: &mut efivar::VarManager) -> LoadOptionIter {
    LoadOptionIter::new(var_manager)
}

pub use operation::get_load_option;
pub use operation::set_boot_next;
