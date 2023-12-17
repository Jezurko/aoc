use std::fs::read_to_string;

pub fn get_map(file: &str) -> Vec< Vec< char > > {
    read_to_string(file).expect("invalid file")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn get_digit_map(file: &str) -> Vec< Vec< isize > > {
    read_to_string(file).expect("invalid file")
        .lines()
        .map(|line| line.chars()
                        .map(|x| x.to_string().parse::< isize >().unwrap())
                        .collect())
        .collect()
}
