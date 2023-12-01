use std::fs::read_to_string;
use regex::{Captures, Regex};

fn get_lines(file: &str) -> String {
    read_to_string(file)
        .expect("invalid file")
}

fn replace_digits(input: &str) -> String {
    let digits = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let replace_digits = |cap: &Captures| -> String {
        match &cap[0] {
            "one" => "1e".to_string(),
            "two" => "2o".to_string(),
            "three" => "3e".to_string(),
            "four" => "4r".to_string(),
            "five" => "5e".to_string(),
            "six" => "6x".to_string(),
            "seven" => "7n".to_string(),
            "eight" => "8t".to_string(),
            "nine" => "9e".to_string(),
            _ => "".to_string()
        }
    };
    let replaced = digits.replace_all(input, replace_digits).to_string();
    return digits.replace_all(replaced.as_str(), replace_digits).to_string();
}

fn load_ints(input: &str) -> Vec< isize > {
    let re = Regex::new(r"[0-9]").unwrap();
    input
        .lines()
        .map(|line|{
            let first_digit = re.find(line).unwrap().as_str();
            let rev = line.chars().rev().collect::< String >();
            let last_digit = re.find(rev.as_str()).unwrap().as_str();
            let digits = [first_digit, last_digit].join("");
            return digits.parse::< isize >().unwrap()
        })
        .collect()
}

fn main() {
   let sum: isize = load_ints(get_lines("inputs/day1.txt").as_str()).iter().sum();
   println!("{}", sum);
   let sum2: isize = load_ints(replace_digits(get_lines("inputs/day1.txt").as_str()).as_str()).iter().sum();
   println!("{}", sum2);
}
