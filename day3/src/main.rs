mod game_map;

fn main() {
    println!("Hello, world!");
    let helper = advent_utils::Helper::new(3);
    helper.print_header();
    let solution = solve(helper.open_file());
}

fn solve(helper: advent_utils::FileHelper) -> u32 {
    let map = game_map::GameMap::new(helper);

    let mut total = 0;
    let mut location = game_map::Location{ x: 0, y: 0 };
    loop {
        let number_opt = map.get_next_number(location);
        if number_opt.is_none() {
            break;
        }

        let number = number_opt.unwrap();

        if map.is_symbol_near(&number) {
            total += number.value();
        }

        location = number.end();
    }

    total
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
}