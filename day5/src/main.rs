mod seed_list;
mod location_map;

fn main() {
    println!("Hello, world!");

    advent_utils::run(5, solve, solve2);
}

fn solve(fh: advent_utils::FileHelper) -> u32 {
    // panic!("Not Implemented")
    42
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