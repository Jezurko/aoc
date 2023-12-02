use std::fs::read_to_string;
use regex::{Captures, Regex};

fn get_lines(file: &str) -> String { read_to_string(file).expect("invalid file") }

fn replace_digits(input: &str) -> String {
    let digits = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let apply_replace = |cap: &Captures| -> &str {
        match &cap[0] {
            "one" => "o1e",
            "two" => "t2o",
            "three" => "t3e",
            "four" => "f4r",
            "five" => "f5e",
            "six" => "s6x",
            "seven" => "s7n",
            "eight" => "e8t",
            "nine" => "n9e",
            _ => ""
        }
    };
    let replaced = digits.replace_all(input, apply_replace);
    return digits.replace_all(&replaced, apply_replace).to_string();
}

fn load_ints(input: &str) -> Vec< isize > {
    let re = Regex::new(r"[0-9]").unwrap();
    input
        .lines()
        .map(|line|{
            let rev = line.chars().rev().collect::< String >();
            let digits = [re.find(line), re.find(&rev)].map(|x|x.unwrap().as_str()).join("");
            return digits.parse::< isize >().unwrap();
        })
        .collect()
}

fn main() {
   println!("{}", load_ints(&get_lines("inputs/day01.txt")).iter().sum::< isize >());
   println!("{}", load_ints(&replace_digits(&get_lines("inputs/day01.txt"))).iter().sum::< isize >());
}
