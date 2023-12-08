use std::{ fs::read_to_string, collections::HashMap, str::{ Lines, Chars }, iter::Cycle };
use num::integer::lcm;
use regex::{Regex};
use lazy_static::lazy_static;

fn get_file(file: &str) -> String { read_to_string(file).expect("invalid file") }
lazy_static!{ static ref LOC:Regex = Regex::new(r"[A-Z]+").unwrap(); }

fn parse_map(lines: Lines) -> HashMap< &str, (&str, &str) >
{
    lines.map(|line| LOC.find_iter(line).map(|loc|loc.as_str()).collect())
         .map(|locs: Vec< &str >| (locs[0], (locs[1], locs[2])))
         .collect()
}

fn solve(start: &str, end: &str, map: &HashMap< &str, (&str, &str) >, directions: &mut Cycle< Chars >) -> usize
{
    let mut loc = start;
    let mut steps = 0;
    while !loc.contains(&end) {
        steps = steps + 1;
        let (left, right) = map.get(loc).unwrap();
        if directions.next().unwrap() == 'L' {loc = left} else {loc  = right};
    }
    return steps;
}

fn part1(mut directions: Cycle< Chars >, map: &HashMap< &str, (&str, &str) >) -> usize
{
    return solve("AAA", "ZZZ", &map, &mut directions);
}

fn part2(mut directions: Cycle< Chars >, map: &HashMap< &str, (&str, &str) >) -> usize
{
    let locs: Vec< &str > = map.keys().filter(|&loc| loc.ends_with("A")).map(|x|*x).collect();
    locs.iter().map(|x| solve(x, "Z", &map, &mut directions)).fold(1, |acc, x| lcm(acc, x))
}

fn main() {
    let input = get_file("inputs/day08.txt");
    let splits: Vec< &str > = input.split("\n\n").collect();
    let directions = splits[0].chars();
    let map = parse_map(splits[1].lines());
    println!("{}", part1(directions.clone().cycle(), &map));
    println!("{}", part2(directions.clone().cycle(), &map));
}
