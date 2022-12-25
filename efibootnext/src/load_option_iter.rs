//! The load option iterator.

use crate::error::GetLoadOptionError;
use crate::Adapter;
use crate::LoadOption;
use std::iter::Iterator;

/// The load option iterator.
pub struct LoadOptionIter<'a, I>
where
    I: Iterator<Item = u16>,
{
    /// The adapter reference.
    adapter: &'a mut Adapter,
    /// The numeric iterator to go over the boot options.
    number_iter: I,
}

impl<'a, I> Iterator for LoadOptionIter<'a, I>
where
    I: Iterator<Item = u16>,
{
    type Item = Result<LoadOption, GetLoadOptionError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let number = match self.number_iter.next() {
                None => return None,
                Some(number) => number,
            };
            match self.adapter.get_load_option(number) {
                Ok(Some(load_option)) => return Some(Ok(load_option)),
                Ok(None) => continue, // skip to the next value
                Err(err) => return Some(Err(err)),
            };
        }
    }
}

impl<'a, I> LoadOptionIter<'a, I>
where
    I: Iterator<Item = u16>,
{
    /// Construct a new [`Self`] with the number iterator.
    pub fn with_number_iter(adapter: &'a mut Adapter, number_iter: I) -> Self {
        Self {
            adapter,
            number_iter,
        }
    }
}
