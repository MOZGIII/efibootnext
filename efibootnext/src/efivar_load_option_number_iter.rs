//! The [`efivar`] load option numbers iterator.

use std::iter::Iterator;

/// [`efivar`] load option numbers iterator.
pub struct EfivarLoadOptionNumberIter<'a> {
    /// The inner iterator.
    pub inner: Box<dyn Iterator<Item = efivar::efi::VariableName> + 'a>,
}

impl<'a> Iterator for EfivarLoadOptionNumberIter<'a> {
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

impl<'a> EfivarLoadOptionNumberIter<'a> {
    /// Create a new [`Self`].
    pub fn new<VarManager>(var_manager: &'a VarManager) -> Result<Self, efivar::Error>
    where
        VarManager: efivar::VarManager + ?Sized,
    {
        let inner = var_manager.get_var_names()?;
        Ok(Self { inner })
    }
}
