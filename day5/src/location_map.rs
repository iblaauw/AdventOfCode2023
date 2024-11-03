use std::str::FromStr;
use advent_utils::generic_error::GenericError;
use advent_utils::parse_utils;

pub struct LocationMap {
    ranges: Vec<LocationMapRange>
}

struct LocationMapRange {
    source_start: u32,
    dest_start: u32,
    size: u32
}

impl LocationMap {
    fn new(_from: &str, _to: &str) -> Self {
        // At some point I should do something with these
        Self {
            ranges: Vec::new()
        }
    }

    pub fn add_range(&mut self, line_range: &str) {
        todo!()
    }

    pub fn map_location(&self, location: u32) -> u32 {
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