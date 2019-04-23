use std::iter::Iterator;

pub struct HeuristicsLoadOptionNumberIter {
    next: u16,
    more: bool,
}

impl Iterator for HeuristicsLoadOptionNumberIter {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.more {
            return None;
        }

        let current = self.next;

        if current == std::u16::MAX {
            self.more = false
        } else {
            self.next = match current {
                0x0020 => 0x9980,
                0x99FF => 0xFF00,
                _ => current + 1,
            };
        }

        return Some(current);
    }
}

impl HeuristicsLoadOptionNumberIter {
    pub fn new() -> Self {
        Self {
            next: 0,
            more: true,
        }
    }
}

#[test]
fn loops_through_all_interesting_values() {
    use std::collections::HashSet;

    let mut interesting_values: HashSet<u16> = HashSet::new();
    interesting_values.insert(0x0000);
    interesting_values.insert(0x0001);
    interesting_values.insert(0x0009);
    interesting_values.insert(0x0010);
    interesting_values.insert(0x9990);
    interesting_values.insert(0x9999);
    interesting_values.insert(0x999F);
    interesting_values.insert(0xFFFF);

    let mut inverse_interesting_values: HashSet<u16> = HashSet::new();
    inverse_interesting_values.insert(0x0025);

    let mut emitted_values: HashSet<u16> = HashSet::new();
    let iter = HeuristicsLoadOptionNumberIter::new();
    let mut iter_count = 0;
    for num in iter {
        iter_count += 1;
        interesting_values.remove(&num);
        assert!(
            emitted_values.insert(num),
            "{:#x} was emitted twice: {:?}",
            num,
            emitted_values
        );
        assert!(
            !inverse_interesting_values.contains(&num),
            "{:#x} was found in the output while it was not expected",
            num
        );
    }

    assert!(
        interesting_values.is_empty(),
        "not all interesing values were emitted: {:?}",
        interesting_values
    );

    let iter_count_limit = 30000;
    assert!(
        iter_count <= iter_count_limit,
        "too many interations, expected {} got {}",
        iter_count_limit,
        iter_count
    );
}
