use std::str::FromStr;
use advent_utils::generic_error::GenericError;
use advent_utils::parse_utils;

pub struct SeedList {
    seeds: Vec<u32>,
}

impl SeedList {
    fn new(seeds : Vec<u32>) -> Self {
        SeedList {
            seeds
        }
    }
}

impl FromStr for SeedList {
    type Err = GenericError;

    fn from_str(line: &str) -> Result<Self, GenericError> {
        let remaining = parse_utils::expect_prefix(line, "seeds:")?;
        let seed_vec = parse_utils::parse_value_list(remaining, ' ')?;
        Ok(SeedList::new(seed_vec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let seeds: SeedList = "seeds: 1 2 3 5 7".parse().expect("Failed to parse test.");
        assert_eq!(seeds.seeds.len(), 5);
        assert_eq!(seeds.seeds[0], 1);
        assert_eq!(seeds.seeds[1], 2);
        assert_eq!(seeds.seeds[2], 3);
        assert_eq!(seeds.seeds[3], 5);
        assert_eq!(seeds.seeds[4], 7);
    }
}