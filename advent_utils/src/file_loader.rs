use std::fs::File;
use std::io::{ BufRead, BufReader, Lines, Read };

pub struct FileLoader {
    day: u32
}

impl FileLoader {
    pub fn new(day: u32) -> FileLoader {
        FileLoader{ day }
    }

    pub fn open_file(&self) -> FileHelper {
        let path = Self::get_file_path(self.day);
        let raw_file = File::open(path).unwrap();

        FileHelper { raw_file }
    }

    fn get_file_path(_: u32) -> String {
        String::from("blah.txt")
    }

}

pub struct FileHelper {
    raw_file: File
}

impl FileHelper {
    pub fn read_all(&mut self) -> String {
        let mut data = String::new();
        self.raw_file.read_to_string(&mut data).unwrap();
        data
    }

    pub fn into_lines(self) -> Lines<BufReader<File>> {
        BufReader::new(self.raw_file).lines()
    }
}
