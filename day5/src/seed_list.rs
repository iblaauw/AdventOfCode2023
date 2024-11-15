use std::str::FromStr;
use advent_utils::generic_error::GenericError;
use advent_utils::parse_utils;
use super::location_map::LocationMap;

pub struct SeedList {
    seeds: Vec<u32>,
}

impl SeedList {
    fn new(seeds : Vec<u32>) -> Self {
        SeedList {
            seeds
        }
    }

    pub fn apply_map(&mut self, map: &LocationMap) {
        for seed in &mut self.seeds {
            *seed = map.map_location(*seed);
        }
    }

    pub fn get_answer(&self) -> Option<u32> {
        self.seeds.iter()
            .min()
            .map(|x| *x)
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