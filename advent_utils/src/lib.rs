mod file_loader;
pub mod grid;
pub mod generic_error;

pub use file_loader::FileHelper;

pub struct Helper {
    day: u32
}

impl Helper {
    pub fn new(day: u32) -> Helper {
        Helper { day }
    }

    pub fn print_header(&self) {
        let day = self.day;
        println!("Beginning day {day} solution:");
        println!("-----------------------------");
    }

    pub fn print_solution<T: std::fmt::Display>(&self, solution: T) {
        let day = self.day;
        println!("Day {day} solution: {solution}");
    }

    pub fn open_file(&self) -> FileHelper {
        let loader = file_loader::FileLoader::new(self.day);
        loader.open_file()
    }

    pub fn open_file_with_name(&self, post_fix: &str) -> FileHelper {
        let loader = file_loader::FileLoader::new(self.day);
        loader.open_file_with_name(post_fix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn end_to_end_fileloader() {
        let helper = Helper::new(5);
        let file = helper.open_file_with_name("foobar");
        let lines: Vec<String> = file.into_lines().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "This is a test");
        assert_eq!(lines[1], "Hopefully the test actually works");
    }
}

pub fn run<S1, S2>(day: u32, solve_func: S1, solve_func2: S2)
    where S1: FnOnce(FileHelper) -> u32,
        S2: FnOnce(FileHelper) -> u32
{
    let helper = Helper::new(day);
    helper.print_header();
    let solution1 = solve_func(helper.open_file());
    helper.print_solution(solution1);
    let solution2 = solve_func2(helper.open_file());
    helper.print_solution(solution2);
}

pub fn test_named<S1, S2>(day: u32, solve_func1: S1, solve_func2: S2, name: &str)
    -> (u32, u32)
    where S1: FnOnce(FileHelper) -> u32,
        S2: FnOnce(FileHelper) -> u32
{
    let helper = Helper::new(day);

    let fh1 = helper.open_file_with_name(name);
    let solution1 = solve_func1(fh1);

    let fh2 = helper.open_file_with_name(name);
    let solution2 = solve_func2(fh2);

    (solution1, solution2)
}

