#[macro_use]
extern crate failure_derive;

pub mod error;
mod heuristics_load_option_number_iter;
mod load_option;
mod load_option_iter;
mod operation;

use heuristics_load_option_number_iter::HeuristicsLoadOptionNumberIter;
use load_option_iter::LoadOptionIter;

pub use failure::Error;
pub use load_option::LoadOption;

pub type Result<T> = std::result::Result<T, Error>;

pub fn load_options<'a>(
    var_manager: &'a mut efivar::VarManager,
) -> impl Iterator<Item = Result<LoadOption>> + 'a {
    let number_iter = HeuristicsLoadOptionNumberIter::new();
    LoadOptionIter::with_number_iter(var_manager, number_iter)
}

pub use operation::get_boot_next;
pub use operation::get_load_option;
pub use operation::set_boot_next;
