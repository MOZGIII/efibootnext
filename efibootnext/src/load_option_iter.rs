use crate::operation::get_load_option;
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
        while self.current <= 0x9999 {
            let load_option = match get_load_option(self.var_manager, self.current) {
                Ok(load_option) => Some(Ok(load_option)),
                Err(err) => {
                    if let Some(NoSuchLoadOption { .. }) = err.downcast_ref() {
                        None
                    } else {
                        Some(Err(err))
                    }
                }
            };

            // Advance counter.
            // We use a bit of heuristic here to try and go over the most
            // popular load options.
            self.current = match self.current {
                0x20 => 0x9980,
                _ => self.current + 1,
            };

            match load_option {
                None => continue,
                Some(val) => return Some(val),
            }
        }
        None
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
