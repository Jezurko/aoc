use std::{ fs::read_to_string, iter::zip};
use regex::{Regex};
use lazy_static::lazy_static;

fn get_file(file: &str) -> String { read_to_string(file).expect("invalid file") }
fn get_str_lines(input: &str) -> Vec< &str > { input.lines().collect() }

lazy_static!{ static ref NUM:Regex = Regex::new(r"[0-9]+").unwrap(); }

fn parse_nums(input: &str) -> Vec< isize >
{
    NUM.find_iter(input).map(|num| num.as_str().parse::< isize >().unwrap()).collect()
}

fn wins(time: isize, dist: isize) -> isize {
    for t in 0..time { if t * (time - t) > dist { return time - 2 * t + 1 }; }
    return 0;
}
fn part1(lines: Vec< &str >) -> isize {
    zip(parse_nums(lines[0]), parse_nums(lines[1])).map(|(t, d)| { wins(t, d)}).product()
}

fn part2(input: &str) -> isize
{
    let race = parse_nums(&input.replace(" ", ""));
    wins(race[0], race[1])
}

fn main() {
    println!("{}", part1(get_str_lines(&get_file("inputs/day06.txt"))));
    println!("{}", part2(&get_file("inputs/day06.txt")));
}
