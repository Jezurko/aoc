use std::fs::read_to_string;
use regex::{Regex};
use lazy_static::lazy_static;
use memoize::memoize;

lazy_static!{ static ref NUM:Regex = Regex::new(r"[0-9]+").unwrap(); }

fn parse_nums(input: &str) -> Vec< usize >
{
    NUM.find_iter(input).map(|num| num.as_str().parse::< usize >().unwrap()).collect()
}

fn get_file(file: &str) -> String { read_to_string(file).expect("invalid file") }
fn get_input(line: &str) -> (&str, Vec< usize >) {
    let splits: Vec< &str > = line.split(' ').collect();
    return (splits[0], parse_nums(splits[1]));
}

#[memoize]
fn resolve_line(springs: String, groups: Vec< usize >) -> usize {
    let springs = springs.chars().collect::< Vec< char > >();
    if springs.len() == 0 { return if groups.is_empty() { 1 } else { 0 }};
    if springs[0] == '.' || (groups.is_empty() && springs[0] == '?') {
        return resolve_line(springs[1..].iter().collect(), groups);
    }
    if groups.is_empty() { return 0; }
    let group = groups[0];
    if group > springs.len() { return 0; }

    let mut sum = 0;
    let next_groups: Vec< usize > = groups[1..].iter().map(|&x|x).collect();
    for i in 0..springs.len() - group + 1 {
        if springs[i..group + i].iter().fold(true, |acc, &x| acc && (x == '?' || x == '#')) {
            if group + i >= springs.len() {
                if next_groups.is_empty() {sum = sum + 1;}
            }
            else if springs[group + i] == '.' || springs[group + i] == '?' {
                sum = sum + resolve_line(springs[group + i + 1..].iter().collect(), next_groups.clone());
            }
        }
        if springs[i] == '#' { break; }
    }
    return sum;
}

fn multiply_input((springs, groups): (&str, &Vec< usize >)) -> (String, Vec< usize >) {
    let long_springs = (springs.to_string() + "?").repeat(4) + springs;
    let long_groups = groups.repeat(5);
    return (long_springs, long_groups);

}

fn main() {
    let input = get_file("inputs/day12.txt");
    let part1 = input.lines().map(|line| get_input(line))
                             .map(|(springs, groups)| resolve_line(springs.to_string(), groups) )
                             .sum::< usize >();
    let part2 = input.lines().map(|line| get_input(line))
                             .map(|(springs, groups)| multiply_input((&springs, &groups)) )
                             .map(|(springs, groups)| resolve_line(springs, groups) )
                             .sum::< usize >();
    println!("Part1: {}, Part2: {}", part1, part2);
}
