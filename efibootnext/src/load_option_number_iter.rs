//! The iterator providing the available load option numbers.

use std::iter::Iterator;

/// The iterator providing the available load option numbers.
pub struct LoadOptionNumberIter<'a> {
    /// The inner iterator.
    pub inner: Box<dyn Iterator<Item = efivar::efi::VariableName> + 'a>,
}

impl<'a> Iterator for LoadOptionNumberIter<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let value = self.inner.next()?;

            let Some(num_str) = value.variable().strip_prefix("Boot") else {
                continue;
            };

            let Ok(num) = num_str.parse() else {
                continue;
            };

            return Some(num);
        }
    }
}

impl<'a> LoadOptionNumberIter<'a> {
    /// Create a new [`Self`].
    pub fn new<VarManager>(var_manager: &'a VarManager) -> Result<Self, efivar::Error>
    where
        VarManager: efivar::VarManager + ?Sized,
    {
        let inner = var_manager.get_var_names()?;
        Ok(Self { inner })
    }
}
