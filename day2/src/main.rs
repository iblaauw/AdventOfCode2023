
fn main() {
    println!("Hello, world!");
    let helper = advent_utils::Helper::new(2);
    helper.print_header();
    let solution = solve(helper.open_file());
    helper.print_solution(solution);
}

fn solve(_fh: advent_utils::FileHelper) -> u32 {
    panic!("Not Implemented");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given() {
        let helper = advent_utils::Helper::new(1);
        let fh = helper.open_file_with_name("given_test");
        let solution = solve(fh);
        assert_eq!(solution, 8);
    }
}
