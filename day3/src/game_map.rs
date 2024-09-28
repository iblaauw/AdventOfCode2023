
pub struct GameMap {
    map: Vec<Vec<char>>
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

pub struct NumberRegion {
    x_start: usize,
    x_end: usize,
    y: usize,
    value: u32,
}

impl GameMap {
    pub fn new(fh: advent_utils::FileHelper) -> Self {
        let grid = advent_utils::grid::create_grid(fh.into_lines());
        Self {
            map: grid
        }
    }

    pub fn get_next_number(&self, start: Location) -> Option<NumberRegion> {
        let mut loc = start;

        let x_len = self.get_x_len();
        let y_len = self.get_y_len();

        while loc.y < y_len {
            while loc.x < x_len {
                let map_item = self.map[loc.y][loc.x];
                if map_item.is_numeric() {
                    let number = self.extract_number(loc);
                    return Some(number)
                }

                loc.x += 1;
            }
            loc.x = 0;
            loc.y += 1;
        }

        return None;
    }

    pub fn is_symbol_near(&self, number: &NumberRegion) -> bool {
        let x_len = self.get_x_len();
        let y_len = self.get_y_len();

        let has_left = number.x_start > 0;
        let has_right = number.x_end < x_len;

        // Get start and end points of the box that the region is determined by but expanded by 1 in
        // all directions. Remember end points are exclusive!
        let start_x = if has_left { number.x_start - 1 } else { 0 };
        let end_x = if has_right { number.x_end + 1 } else { x_len };

        let top_y = if number.y == 0 { None } else { Some(number.y - 1) };
        let bottom_y = if number.y + 1 >= y_len { None } else { Some(number.y + 1) };

        fn counts_as_symbol(val: char) -> bool {
            // Be careful... another number doesn't count as a symbol
            val != '.' && !val.is_alphanumeric()
        }

        // Check the top row (if any)
        if let Some(y) = top_y {
            for x in start_x..end_x {
                let map_item = self.map[y][x];
                if counts_as_symbol(map_item) {
                    return true;
                }
            }
        }

        // Check the bottom row (if any)
        if let Some(y) = bottom_y {
            for x in start_x..end_x {
                let map_item = self.map[y][x];
                if counts_as_symbol(map_item) {
                    return true;
                }
            }
        }

        // Check the sides now

        if has_left {
            let map_item = self.map[number.y][start_x];
            if counts_as_symbol(map_item) {
                return true;
            }
        }

        if has_right {
            // end_x is *exclusive* end point
            let map_item = self.map[number.y][end_x - 1];
            if counts_as_symbol(map_item) {
                return true;
            }
        }
        

        return false;
    }

    pub fn get_adjacent_symbols(&self, number: &NumberRegion, symbol: char) -> impl Iterator<Item = Location> {
        panic!("Not Impl");
        Vec::<Location>::new().into_iter()
    }

    fn get_x_len(&self) -> usize {
        if self.get_y_len() == 0 { 0 } else { self.map[0].len() }
    }

    fn get_y_len(&self) -> usize {
        self.map.len()
    }

    fn extract_number(&self, loc: Location) -> NumberRegion {
        let remaining_slice = &self.map[loc.y][(loc.x)..];
        let slice_len = remaining_slice.len();
        let end_index = remaining_slice
            .iter()
            .position(|c| !c.is_numeric())
            .unwrap_or(slice_len); // Having 'None' means that the entirety of the slice is numeric.

        // Unfortunately we can't convert from a char slice to a &str without going through a String (introduces an allocation + copy).
        // TODO: I could write a helper, but let's validate it works first.
        let value_str = String::from_iter(&remaining_slice[..end_index]);

        let value: u32 = value_str.parse().expect("Couldn't covert a str of only digits to a number??");
        
        NumberRegion {
            x_start: loc.x,
            x_end: loc.x + end_index,
            y: loc.y,
            value
        }
    }
}

impl Copy for Location {}

impl NumberRegion {
    pub fn start(&self) -> Location {
        Location { 
            x: self.x_start,
            y: self.y,
        }
    }

    pub fn end(&self) -> Location {
        Location { 
            x: self.x_end,
            y: self.y,
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
