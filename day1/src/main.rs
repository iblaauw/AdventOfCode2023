fn main() {
    println!("Hello, world!");
    println!("Uh oh :O");

    let helper = advent_utils::Helper::new(1);
    helper.print_header();
    let solution = solve(helper.open_file());
    helper.print_solution(solution);
}

fn solve(fh: advent_utils::FileHelper) -> u32 {
    fh.into_lines()
        .map(|l| get_number(&l))
        .sum()
}

fn get_number(line: &str) -> u32 {
    panic!("Not Implemented!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers_basic() {
        assert_eq!(get_number("1abc2"), 12);
        assert_eq!(get_number("9abc9"), 99);
        assert_eq!(get_number("4fooobardks5"), 45);
    }

    #[test]
    fn test_numbers_hard() {
        assert_eq!(get_number("zz1abc2xy"), 12);
        assert_eq!(get_number("9alphabet9cc"), 99);
        assert_eq!(get_number("4fo2oob33ardks5"), 45);
        assert_eq!(get_number("catastrophie23"), 23);
        assert_eq!(get_number("21uandmebaby"), 21);
        assert_eq!(get_number("2bland4u"), 24);
        assert_eq!(get_number("and4ever2gethermakes5tobecome1yay"), 41);
    }

    #[test]
    fn test_numbers_single() {
        assert_eq!(get_number("good2go"), 22);
        assert_eq!(get_number("probl9ems"), 99);
        assert_eq!(get_number("6"), 66);
        assert_eq!(get_number("5b"), 55);
        assert_eq!(get_number("b4"), 44);
    }

    #[test]
    fn test_numbers_given() {
        assert_eq!(get_number("1abc2"), 12);
        assert_eq!(get_number("pqr3stu8vwx"), 38);
        assert_eq!(get_number("a1b2c3d4e5f"), 15);
        assert_eq!(get_number("treb7uchet"), 77);
    }

    #[test]
    fn test_solution_given() {
        let helper = advent_utils::Helper::new(1);
        let fh = helper.open_file_with_name("given_test");
        let solution = solve(fh);
        assert_eq!(solution, 142);
    }
}
