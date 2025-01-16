use std::collections::VecDeque;
use std::str::FromStr;
use advent_utils::generic_error::GenericError;
use advent_utils::parse_utils;
use std::ops::Range;

pub struct LocationMap {
    ranges: Vec<LocationMapRange>,
    debug_flag: bool,
}

struct LocationMapRange {
    source_start: u32,
    dest_start: u32,
    size: u32,
}

pub struct PartialRangeMatch {
    pub result: Option<Range<u32>>,
    pub remaining: Option<Range<u32>>
}

enum RangeSplitResult {
    NoOverlap,
    FullyContained,
    SplitFront(Range<u32>, Range<u32>), // The first piece was contained
    SplitBack(Range<u32>, Range<u32>), // The second piece was contained
    Encompassed(Range<u32>, Range<u32>, Range<u32>)
}

impl LocationMap {
    fn new(_from: &str, _to: &str) -> Self {
        // At some point I should do something with these
        Self {
            ranges: Vec::new(),
            debug_flag: false,
        }
    }

    pub fn add_range(&mut self, line_range: &str) -> Result<(), GenericError> {
        let range: LocationMapRange = line_range.parse()?;
        self.ranges.push(range);

        Ok(())
    }

    pub fn map_location(&self, location: u32) -> u32 {
        if self.debug_flag {
            println!("Debug: Begin map");
        }

        for range in &self.ranges {
            if self.debug_flag {
                println!("Debug:     range is - Dest {}, Source {}, Size {}", range.dest_start, range.source_start, range.size);
            }
            let new_loc = range.map_location(location);
            if new_loc.is_some() {
                return new_loc.unwrap();
            }
        }

        return location;
    }

    pub fn set_debug_flag(&mut self) {
        self.debug_flag = true;
    }

    pub fn map_range(&self, location_range: &Range<u32>) -> Vec<Range<u32>> {
        let mut result = Vec::new();
        let mut processing_queue = VecDeque::new();
        processing_queue.push_back(location_range);

        while let Some(value) = processing_queue.pop_front() {
            let processed = true;

            // Find the first one that doesn't
            self.ranges.iter()
                .map(|r| r.split_range(location_range))
                .nth(0);
            for map_range in &self.ranges {
                match map_range.split_range(value) {
                    RangeSplitResult::NoOverlap => {},

                }
            }
        }

        return result;
    }

    pub fn map_partial_range(&self, location_range: Range<u32>) -> PartialRangeMatch {
        todo!()
    }
}

impl FromStr for LocationMap {
    type Err = GenericError;

    fn from_str(line: &str) -> Result<Self, GenericError> {
        let line = line.trim();
        let remaining = parse_utils::expect_suffix(line, " map:")?;
        let splits: Vec<&str> = remaining.split('-').collect();
        if splits.len() != 3 || splits[1] != "to" {
            return GenericError::as_err(format!("Unexpected map name {}", remaining));
        }

        Ok(LocationMap::new(splits[0], splits[2]))
    }
}

impl LocationMapRange {
    fn map_location(&self, value: u32) -> Option<u32> {
        if value < self.source_start || (value - self.source_start) > self.size {
            return None;
        } else {
            return Some(self.dest_start + (value - self.source_start));
        }
    }

    fn split_range(&self, range: &Range<u32>) -> RangeSplitResult {
        let source = self.source_range();

        if source.contains(&range.start) {
            if source.contains(&(range.end - 1)) {
                RangeSplitResult::FullyContained
            } else {
                let piece1 = range.start..source.end;
                let piece2 = source.end..range.end;
                RangeSplitResult::SplitFront(piece1, piece2)
            }
        } else if range.contains(&source.start) {
            if range.contains(&(source.end - 1)) {
                let piece1 = range.start..source.start;
                let piece2 = source.end..range.end;
                RangeSplitResult::Encompassed(piece1, source, piece2)
            } else {
                let piece1 = source.start..range.end;
                let piece2 = range.end..source.end;
                RangeSplitResult::SplitBack(piece1, piece2)
            }
        } else {
            RangeSplitResult::NoOverlap
        }
    }

    fn map_partial_range(&self, range: Range<u32>) -> PartialRangeMatch {
        let source = self.source_range();
        
        if source.contains(&range.start) {

        } else {

        }

        todo!()
    }

    fn source_range(&self) -> Range<u32> {
        self.source_start..(self.source_start + self.size)
    }

    fn dest_range(&self) -> Range<u32> {
        self.dest_start..(self.dest_start + self.size)
    }
}

impl FromStr for LocationMapRange {
    type Err = GenericError;

    fn from_str(line: &str) -> Result<Self, GenericError> {
        let line = line.trim();
        let values: Vec<u32> = parse_utils::parse_value_list(line, ' ')?;
        if values.len() != 3 {
            return GenericError::as_err(format!("Invalid location map range {}", line));
        }

        Ok(
            LocationMapRange {
                source_start: values[1],
                dest_start: values[0],
                size: values[2],
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating() {
        let mut map: LocationMap = "alpha-to-omega map:".parse().expect("Failed to parse test");
        map.add_range("5 12 42").expect("Failed to add first range");
        map.add_range("8 21 37").expect("Failed to add second range");

        assert_eq!(map.ranges.len(), 2);

        let first_range = &map.ranges[0];
        assert_eq!(first_range.source_start, 12);
        assert_eq!(first_range.dest_start, 5);
        assert_eq!(first_range.size, 42);

        let second_range = &map.ranges[1];
        assert_eq!(second_range.source_start, 21);
        assert_eq!(second_range.dest_start, 8);
        assert_eq!(second_range.size, 37);
    }

    #[test]
    fn map_range() {
        let map_range: LocationMapRange = "42 59 7".parse().expect("Failed to parse map");
        
        assert_eq!(map_range.map_location(3), None);
        assert_eq!(map_range.map_location(59), Some(42));
        assert_eq!(map_range.map_location(42), None);
        assert_eq!(map_range.map_location(66), Some(49));
        assert_eq!(map_range.map_location(67), None);
        assert_eq!(map_range.map_location(58), None);
        assert_eq!(map_range.map_location(63), Some(46));
    }

    #[test]
    fn map_all() {
        let mut map: LocationMap = "blob-to-square map:".parse().expect("Fail to parse map");
        map.add_range("23 11 4").expect("Failed to add first range");
        map.add_range("70 50 3").expect("Failed to add second range");
        map.add_range("4 75 10").expect("Failed to add third range");

        assert_eq!(map.map_location(42), 42);
        assert_eq!(map.map_location(0), 0);
        assert_eq!(map.map_location(10), 10);
        assert_eq!(map.map_location(51), 71);
        assert_eq!(map.map_location(80), 9);
        assert_eq!(map.map_location(15), 27);
    }
}