mod file_loader;

pub struct Helper {
    day: u32
}

impl Helper {
    pub fn new(day: u32) -> Helper {
        Helper { day }
    }

    pub fn open_file(&self) -> file_loader::FileHelper {
        let loader = file_loader::FileLoader::new(self.day);
        loader.open_file()
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
