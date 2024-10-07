mod game_map;
mod gear_store;

fn main() {
    println!("Hello, world!");
    let helper = advent_utils::Helper::new(3);
    helper.print_header();
    // let solution = solve(helper.open_file());
    let solution = solve(helper.open_file());
    helper.print_solution(solution);

    let solution2 = solve2(helper.open_file());
    helper.print_solution(solution2);
}

fn solve(helper: advent_utils::FileHelper) -> u32 {
    let map = game_map::GameMap::new(helper);

    let mut total = 0;
    let mut location = game_map::Location{ x: 0, y: 0 };
    loop {
        println!("Looking for number...");
        let number_opt = map.get_next_number(location);
        if number_opt.is_none() {
            println!("Done!");
            break;
        }

        let number = number_opt.unwrap();

        println!("Got number {}", number.value());

        if map.is_symbol_near(&number) {
            println!("  And it's near a symbol!");
            total += number.value();
        }

        location = number.end();
    }

    total
}

fn solve2(helper: advent_utils::FileHelper) -> u32 {
    let map = game_map::GameMap::new(helper);

    let mut gear_store = gear_store::GearStore::new();

    let mut location = game_map::Location{ x: 0, y: 0 };
    loop {
        let number_opt = map.get_next_number(location);
        if number_opt.is_none() {
            break;
        }

        let number = number_opt.unwrap();

        for gear_loc in map.get_adjacent_symbols(&number, '*') {
            gear_store.add(gear_loc, number.value());
        }

        location = number.end();
    }

    gear_store.calculate_ratios()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given() {
        let helper = advent_utils::Helper::new(3);
        let fh = helper.open_file_with_name("given_test");
        let solution = solve(fh);
        assert_eq!(solution, 4361);
    }

    #[test]
    fn test_given2() {
        let helper = advent_utils::Helper::new(3);
        let fh = helper.open_file_with_name("given_test");
        let solution = solve2(fh);
        assert_eq!(solution, 467835);
    }
}