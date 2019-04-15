use crate::operation::load_option_by_num;
use crate::LoadOption;
use crate::NoSuchLoadOption;
use efivar;
use failure::Error;
use std::iter::Iterator;

pub struct LoadOptionIter<'a> {
    var_manager: &'a mut dyn efivar::VarManager,
    current: u16,
}

impl<'a> Iterator for LoadOptionIter<'a> {
    type Item = Result<LoadOption, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let load_option = match load_option_by_num(self.var_manager, self.current) {
            Ok(load_option) => load_option,
            Err(err) => {
                if let Some(NoSuchLoadOption { .. }) = err.downcast_ref() {
                    return None;
                }
                return Some(Err(err));
            }
        };
        self.current += 1;
        Some(Ok(load_option))
    }
}

impl<'a> LoadOptionIter<'a> {
    pub fn new(var_manager: &'a mut dyn efivar::VarManager) -> Self {
        Self {
            var_manager: var_manager,
            current: 0,
        }
    }
}
