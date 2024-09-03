
pub struct GameMap {
    map: Vec<Vec<char>>
}

#[derive(Clone)]
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
            }
            loc.x = 0;
            loc.y += 1;
        }

        return None;
    }

    pub fn is_symbol_near(&self, number: &NumberRegion) -> bool {
        panic!("Not Implemented");
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
