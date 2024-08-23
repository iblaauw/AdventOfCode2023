mod game_input;

use game_input::Game;

fn main() {
    println!("Hello, world!");
    let helper = advent_utils::Helper::new(2);
    helper.print_header();
    let solution = solve(helper.open_file());
    helper.print_solution(solution);
}

fn solve(fh: advent_utils::FileHelper) -> u32 {

    let mut result = 0;
    for line in fh.into_lines() {
        let game: Game = line.parse().unwrap();
        if is_valid(&game) {
            result += game.id;
        }
    }

    result
}

fn is_valid(game: &Game) -> bool {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    for d in &game.draws {
        if (d.red > MAX_RED) ||
            (d.green > MAX_GREEN) ||
            (d.blue > MAX_BLUE) {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given() {
        let helper = advent_utils::Helper::new(2);
        let fh = helper.open_file_with_name("given_test");
        let solution = solve(fh);
        assert_eq!(solution, 8);
    }
}
