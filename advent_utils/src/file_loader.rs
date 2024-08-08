use std::fs::File;
use std::io::{ BufRead, BufReader, Read, Seek };

pub struct FileLoader {
    day: u32
}

impl FileLoader {
    pub fn new(day: u32) -> FileLoader {
        FileLoader{ day }
    }

    pub fn open_file(&self) -> FileHelper {
        self.open_file_worker(None)
    }

    pub fn open_file_with_name(&self, post_fix: &str) -> FileHelper {
        self.open_file_worker(Some(post_fix))
    }

    fn open_file_worker(&self, post_fix: Option<&str>) -> FileHelper {
        let path = self.get_file_path(post_fix);
        let raw_file = File::open(path).unwrap();
        FileHelper { raw_file }
    }

    fn get_file_path(&self, post_fix: Option<&str>) -> String {
        let day = self.day;
        match post_fix {
            Some(pf) => format!("data/day{day}_{pf}.txt"),
            None => format!("data/day{day}.txt")
        }
    }
}

pub struct FileHelper {
    raw_file: File
}

impl FileHelper {
    pub fn read_all(&mut self) -> String {
        let mut data = String::new();
        self.raw_file.read_to_string(&mut data).unwrap();
        self.raw_file.rewind().unwrap();
        data
    }

    pub fn into_string(mut self) -> String {
        let mut data = String::new();
        self.raw_file.read_to_string(&mut data).unwrap();
        data
    }

    pub fn into_lines(self) -> impl Iterator<Item = String> {
        BufReader::new(self.raw_file).lines().map(|r| r.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_lines() {
        const FILENAME : &str = "data/fileTest1.txt";
        let f = File::open(FILENAME).unwrap();
        let fh = FileHelper{ raw_file: f };
        let lines: Vec<String> = fh.into_lines().collect();

        assert!(lines.len() == 4);
        assert_eq!(lines[0], "alphabet");
        assert_eq!(lines[1], "soup");
        assert_eq!(lines[2], "is");
        assert_eq!(lines[3], "delicious");
    }

    #[test]
    fn test_read_all() {
        const FILENAME : &str = "data/fileTest2.txt";
        const CONTENTS: &str = "I love animals because they  are fun!";
        let f = File::open(FILENAME).unwrap();
        let mut fh = FileHelper{ raw_file: f };
        let contents = fh.read_all();
        assert_eq!(contents, CONTENTS);
        let contents2 = fh.read_all();
        assert_eq!(contents2, CONTENTS);
    }

    #[test]
    fn test_into_string() {
        const FILENAME : &str = "data/fileTest2.txt";
        const CONTENTS: &str = "I love animals because they  are fun!";
        let f = File::open(FILENAME).unwrap();
        let fh = FileHelper{ raw_file: f };

        let contents = fh.into_string();
        assert_eq!(contents, CONTENTS);
    }

    #[test]
    fn test_postfix() {
        let fl = FileLoader{ day: 13 };
        assert_eq!(fl.get_file_path(None), "data/day13.txt");
        assert_eq!(fl.get_file_path(Some("fooblah")), "data/day13_fooblah.txt");
        assert_eq!(fl.get_file_path(Some("food.good")), "data/day13_food.good.txt");
    }
}
