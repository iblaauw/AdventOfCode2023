use std::{collections::VecDeque, ops::Range};

pub struct LocationRange {
    range: Range<u32>
}

pub enum SplitResult {
    Same(LocationRange),
    Split(LocationRange, LocationRange),
}

struct SplitIter {
    first: Option<LocationRange>,
    second: Option<LocationRange>
}

impl LocationRange {
    pub fn new(start: u32, size: u32) -> Self {
        Self {
            range: start..(start+size)
        }
    }

    pub fn contains(&self, val: u32) -> bool {
        self.range.contains(&val)
    }

    pub fn split(self, val: u32) -> SplitResult {
        if self.range.contains(&val) {
            SplitResult::Split(
                (self.range.start..val).into(),
                (val..self.range.end).into()
            )
        } else {
            SplitResult::Same(self)
        }
    }

    pub fn split_iter(self, val: u32) -> SplitIter {
        SplitIter::new(self.split(val))
    }

    // TODO: make this an iterable
    pub fn split_range(self, other: &LocationRange) -> VecDeque<LocationRange> {
        let mut result = VecDeque::new();

        match self.split(other.range.start) {
            SplitResult::Same(same_val) => result.push_back(same_val),
            SplitResult::Split(piece1, piece2 ) => {
                result.push_back(piece1);
                result.push_back(piece2);
            }
        }

        let mut found = false;
        let first = result.pop_front().unwrap();
        match first.split(other.range.end - 1) {
            SplitResult::Same(same_val) => result.push_front(same_val),
            SplitResult::Split(piece1, piece2 ) => {
                found = true;
                // Push front (in reverse order)
                result.push_front(piece2);
                result.push_front(piece1);
            }
        }

        if !found {
            let last = result.pop_back().unwrap();
            match last.split(other.range.end - 1) {
                SplitResult::Same(same_val) => result.push_back(same_val),
                SplitResult::Split(piece1, piece2) => {
                    result.push_back(piece1);
                    result.push_back(piece2);
                }
            }
        }

        return result;
    }
}

impl From<Range<u32>> for LocationRange {
    fn from(value: Range<u32>) -> Self {
        Self {
            range: value
        }
    }
}

impl SplitIter {
    fn new(result: SplitResult) -> Self {
        let mut myself = Self {
            first: None,
            second: None
        };

        match result {
            SplitResult::Same(val) => {
                myself.first = Some(val);
            },
            SplitResult::Split(piece1, piece2 ) => {
                myself.first = Some(piece1);
                myself.second = Some(piece2);
            }
        }

        return myself;
    }
}

impl Iterator for SplitIter {
    type Item = LocationRange;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first.is_some() {
            self.first.take()
        } else {
            self.second.take()
        }
    }
}
