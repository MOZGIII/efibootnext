use crate::operation::get_load_option;
use crate::LoadOption;
use crate::NoSuchLoadOption;
use crate::Result;
use efivar;
use std::iter::Iterator;

pub struct LoadOptionIter<'a, I>
where
    I: Iterator<Item = u16>,
{
    var_manager: &'a mut dyn efivar::VarManager,
    number_iter: I,
}

impl<'a, I> Iterator for LoadOptionIter<'a, I>
where
    I: Iterator<Item = u16>,
{
    type Item = Result<LoadOption>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let number = match self.number_iter.next() {
                None => return None,
                Some(number) => number,
            };
            match get_load_option(self.var_manager, number) {
                Ok(load_option) => return Some(Ok(load_option)),
                Err(err) => {
                    if let Some(NoSuchLoadOption { .. }) = err.downcast_ref() {
                        continue;
                    } else {
                        return Some(Err(err));
                    }
                }
            };
        }
    }
}

impl<'a, I> LoadOptionIter<'a, I>
where
    I: Iterator<Item = u16>,
{
    pub fn with_number_iter(var_manager: &'a mut dyn efivar::VarManager, number_iter: I) -> Self {
        Self {
            var_manager: var_manager,
            number_iter: number_iter,
        }
    }
}
