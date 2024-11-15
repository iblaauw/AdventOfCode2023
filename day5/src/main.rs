use std::borrow::Borrow;

mod seed_list;
mod location_map;

fn main() {
    println!("Hello, world!");

    advent_utils::run(5, solve, solve2);
}

fn solve(fh: advent_utils::FileHelper) -> u32 {
    let lines : Vec<String> = fh.into_lines().collect();
    if lines.len() == 0 {
        panic!("Empty input??");
    }

    let mut seeds : seed_list::SeedList = lines[0].parse().expect("Invalid seed list");

    // Walk through and find each map and apply it
    let mut line_index = 1;
    let mut current_map: Option<location_map::LocationMap> = None;
    while line_index < lines.len() {

        let line = lines[line_index].trim();
        if line.is_empty() {
            // Skip empty lines
            line_index += 1;
        }
        else
        {
            if line.ends_with(':') {
                if let Some(map) = &current_map {
                    println!("    Applying to seeds...");
                    // Our previous map is complete. Apply it to our seed list.
                    seeds.apply_map(map);
                }

                println!("Processing new map: {}", line);

                // This is a map, let's parse it
                let new_map : location_map::LocationMap = line.parse::<location_map::LocationMap>()
                    .expect(&format!("Invalid map? Line: {}", line));

                current_map = Some(new_map);
            }
            else
            {
                // Add a range
                let map = current_map.as_mut().expect(&format!("Error: found a range line with no valid map? Line: {}", line));
                map.add_range(line).expect(&format!("Invalid range? Line: {}", line));
            }

            line_index += 1;
        }

    }

    // Apply the last map
    if let Some(map) = &mut current_map {
        map.set_debug_flag();
        println!("    Final applying to seeds...");
        seeds.apply_map(map);
    }

    return seeds.get_answer().expect("No seeds???");
}

fn solve2(fh: advent_utils::FileHelper) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given() {
        let (s1, s2) = advent_utils::test_named(5, solve, solve2, "given");
        assert_eq!(s1, 35);
        assert_eq!(s2, 0);
    }
}