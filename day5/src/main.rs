fn main() {
    println!("Hello, world!");

    let helper = advent_utils::Helper::new(5);
    helper.print_header();
    let solution = solve(helper.open_file());
    helper.print_solution(solution);

    let solution2 = solve2(helper.open_file());
    helper.print_solution(solution2);
}

fn solve(fh: advent_utils::FileHelper) -> u32 {
    // panic!("Not Implemented")
    42
}

fn solve2(fh: advent_utils::FileHelper) -> u32 {
    0
}