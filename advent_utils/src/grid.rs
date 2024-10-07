
pub fn create_grid(line_iter: impl Iterator<Item = String>) -> Vec<Vec<char>> {
    let mut vec: Vec<Vec<char>> = Vec::new();
    for line in line_iter {
        let strip: Vec<char> = line.chars().collect();
        vec.push(strip);
    }
    vec
}