use std::slice::SliceIndex;

fn main() {
    println!("Hello, world!");
    println!("Uh oh :O");

    let helper = advent_utils::Helper::new(1);
    helper.print_header();
    let solution = solve(helper.open_file());
    helper.print_solution(solution);

    let solution2 = solve2(helper.open_file());
    helper.print_solution(solution2);
}

fn solve(fh: advent_utils::FileHelper) -> u32 {
    fh.into_lines()
        .map(|l| get_number(&l))
        .sum()
}

fn get_number(line: &str) -> u32 {
    let first_index = line.find(char::is_numeric).expect("Couldn't find any digits?");
    let last_index = line.rfind(char::is_numeric).expect("Couldn't find any digits?");
    let first_digit: u32 = line[first_index..(first_index+1)].parse().expect("Digit we found wasn't an int?");
    let last_digit: u32 = line[last_index..(last_index+1)].parse().expect("Digit we found wasn't an int?");
    let result = (first_digit * 10) + last_digit;
    result as u32
}

fn solve2(fh: advent_utils::FileHelper) -> u32 {
    fh.into_lines()
        .map(|l| get_number2(&l))
        .sum()
}

const NUM_LITERALS: [&str;9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn get_number2(line: &str) -> u32 {
    if let Some((i1, i2)) = get_numeric_indices(line) {
        let prefix = &line[..i1];
        let first_num;
        if let Some(r1) = find_number_forward(prefix) {
            let piece = r1.slice(prefix);
            first_num = number_parse(piece);
        } else {
            first_num = line[]
        }

        panic!("Not Implemented");
    } else {
        panic!("Not Implemented");
    }
}

fn get_numeric_indices(line: &str) -> Option<(usize, usize)> {
    let first = line.find(char::is_numeric);

    let first_index = first?;
    let second = line.rfind(char::is_numeric);
    let second_index = second.expect("Could find a digit in one direction but not the other?");
    Some((first_index, second_index))
}

fn number_parse(line: &str) -> Option<u32> {
    match line {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None
    }
}

fn find_number_forward(line: &str) -> Option<RangeIndex> {
    let mut vals: Option<RangeIndex> = None;
    for number in NUM_LITERALS {
        if let Some(range) = RangeIndex::find(line, number) {
            let replace = match &vals {
                None => true,
                Some(v) => range.start < v.start
            };
            if replace {
                vals = Some(range);
            }
        }
    }
    vals
}

fn temp(vals: Option<RangeIndex>, line: &str) -> Option<u32> {
    let range = vals?;
    let substr = range.slice(line);
    let number = number_parse(substr).expect("Can't parse a found number?");
    Some(number)
}

struct RangeIndex {
    start: usize,
    stop: usize
}

impl RangeIndex {
    fn slice<'a>(&self, input: &'a str) -> &'a str {
        &input[(self.start)..(self.stop)]
    }

    fn find(input: &str, pattern: &str) -> Option<Self> {
        let index = input.find(pattern)?;
        let range = RangeIndex {
            start: index,
            stop: index + pattern.len(),
        };
        Some(range)
    }

    fn rfind(input: &str, pattern: &str) -> Option<Self> {
        let index = input.rfind(pattern)?;
        let range = RangeIndex {
            start: index,
            stop: index + pattern.len(),
        };
        Some(range)
    }

    /*fn find_num(input: &str) -> Option<Self> {
        let index = input.find(char::is_numeric)?;
        let range = RangeIndex {
            start: index, stop: index+1
        };
        Some(range)
    }

    fn rfind_num(input: &str) -> Option<Self> {
        let index = input.find(char::is_numeric)?;
        let range = RangeIndex {
            start: index, stop: index+1
        };
        Some(range)
    }*/
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
    fn test_numbers_basic2() {
        assert_eq!(get_number2("one2three"), 13);
        assert_eq!(get_number2("four5"), 45);
        assert_eq!(get_number2("eight"), 88);
        assert_eq!(get_number2("fivenine2"), 52);
        assert_eq!(get_number2("six2six2"), 62);
        assert_eq!(get_number2("twooutofseven"), 27);
        assert_eq!(get_number2("sevenhundredand923five7"), 77);
    }

    #[test]
    fn test_solution_given() {
        let helper = advent_utils::Helper::new(1);
        let fh = helper.open_file_with_name("given_test");
        let solution = solve(fh);
        assert_eq!(solution, 142);
    }

    #[test]
    fn test_solution_given2() {
        let helper = advent_utils::Helper::new(1);
        let fh = helper.open_file_with_name("given_test2");
        let solution = solve2(fh);
        assert_eq!(solution, 281);
    }

}
