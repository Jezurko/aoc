use std::fs::read_to_string;

pub fn get_map(file: &str) -> Vec< Vec< char > > {
    read_to_string(file).expect("invalid file")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
