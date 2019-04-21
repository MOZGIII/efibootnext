use std::iter::Iterator;

pub struct HeuristicsLoadOptionNumberIter {
    current: u16,
}

impl Iterator for HeuristicsLoadOptionNumberIter {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > 0x9999 {
            return None;
        }

        self.current = match self.current {
            0x20 => 0x9980,
            _ => self.current + 1,
        };

        return Some(self.current);
    }
}

impl HeuristicsLoadOptionNumberIter {
    pub fn new() -> Self {
        Self { current: 0 }
    }
}
