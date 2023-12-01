use std::fs::read_to_string;
use regex::{Captures, Regex};

fn get_lines(file: &str) -> String {
    read_to_string(file)
        .expect("invalid file")
}

fn replace_digits(input: &str) -> String {
    let digits = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let replace_digits = |cap: &Captures| -> &str {
        match &cap[0] {
            "one" => "1e",
            "two" => "2o",
            "three" => "3e",
            "four" => "4r",
            "five" => "5e",
            "six" => "6x",
            "seven" => "7n",
            "eight" => "8t",
            "nine" => "9e",
            _ => ""
        }
    };
    let replaced = digits.replace_all(input, replace_digits);
    return digits.replace_all(&replaced, replace_digits).to_string();
}

fn load_ints(input: &str) -> Vec< isize > {
    let re = Regex::new(r"[0-9]").unwrap();
    input
        .lines()
        .map(|line|{
            let first_digit = re.find(line).unwrap().as_str();
            let rev = line.chars().rev().collect::< String >();
            let last_digit = re.find(&rev).unwrap().as_str();
            let digits = [first_digit, last_digit].join("");
            return digits.parse::< isize >().unwrap()
        })
        .collect()
}

fn main() {
   let sum: isize = load_ints(&get_lines("inputs/day1.txt")).iter().sum();
   println!("{}", sum);
   let sum2: isize = load_ints(&replace_digits(&get_lines("inputs/day1.txt"))).iter().sum();
   println!("{}", sum2);
}
