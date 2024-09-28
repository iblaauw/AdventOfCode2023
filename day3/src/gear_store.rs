use std::collections::HashMap;

use crate::game_map;


pub struct GearStore {
    gear_map: HashMap<game_map::Location, GearData>
}

struct GearData {
    count: u32,
    values: [u32; 2]
}

impl GearStore {
    pub fn new() -> GearStore {
        GearStore {
            gear_map: HashMap::new()
        }
    }

    pub fn add(&mut self, location: game_map::Location, value: u32) {
        self.gear_map.entry(location)
            .or_insert_with(|| GearData::new())
            .add(value);
    }

    pub fn calculate_ratios(&self) -> u32 {
        self.gear_map.values()
            .map(|d| d.get_ratio())
            .sum()
    }
}

impl GearData { 
    fn new() -> GearData {
        GearData {
            count: 0,
            values: [0, 0]
        }
    }

    fn add(&mut self, value: u32) {
        self.count += 1;
        match self.count {
            1 => self.values[0] = value,
            2 => self.values[1] = value,
            _ => {} // Intentionally do nothing otherwise
        }
    }

    fn get_ratio(&self) -> u32 {
        if self.count > 0 && self.count <= 2 {
            self.values[0] + self.values[1]
        } else {
            0
        }
    }
}
