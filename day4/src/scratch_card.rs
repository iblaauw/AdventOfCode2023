use advent_utils::generic_error::GenericError;
use std::collections::HashSet;
use std::ops::Index;
use std::str::FromStr;

pub struct ScratchCard {
    id: u32,
    winners: HashSet<u32>,
    values: HashSet<u32>,
}

impl ScratchCard {
    pub fn get_value(&self) -> u32 {
        panic!("Not Impl");
    }
}

impl FromStr for ScratchCard {
    type Err = GenericError;

    fn from_str(line: &str) -> Result<Self, GenericError> {
        let card = ScratchCard {
            id: 0,
            winners: HashSet::new(),
            values: HashSet::new(),
        };

        let split = line.split_once(':');
        if split.is_none() {
            return GenericError::as_err("Invalid line - found no ':' symbol".into());
        }

        let (id_piece, remaining) = split.unwrap();

        panic!("Not Impl");
    }
}
