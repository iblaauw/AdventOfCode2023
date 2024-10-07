fn main() {
    println!("Hello, world!");

    let helper = advent_utils::Helper::new(4);
    helper.print_header();
    let solution = solve(helper.open_file());
    helper.print_solution(solution);

    let solution2 = solve2(helper.open_file());
    helper.print_solution(solution2);
}

fn solve(helper: advent_utils::FileHelper) -> u32 {
    panic!("Not Impl");
}

fn solve2(helper: advent_utils::FileHelper) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given() {
        let helper = advent_utils::Helper::new(4);
        let fh = helper.open_file_with_name("given_test");
        let solution = solve(fh);
        assert_eq!(solution, 13);
    }
}
