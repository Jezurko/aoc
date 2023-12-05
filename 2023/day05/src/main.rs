use std::fs::read_to_string;
use regex::{Regex};
use lazy_static::lazy_static;

fn get_file(file: &str) -> String { read_to_string(file).expect("invalid file") }
fn get_str_lines(input: &str) -> Vec< &str > { input.lines().collect() }

lazy_static!{ static ref NUM:Regex = Regex::new(r"[0-9]+").unwrap(); }

fn parse_nums(input: &str) -> Vec< isize >
{
    NUM.find_iter(input).map(|num| num.as_str().parse::< isize >().unwrap()).collect()
}


fn get_seeds(line: &str) -> Vec< isize >
{
    parse_nums(line.split(':').nth(1).unwrap())
}

fn get_mapping(lines: Vec< &str >) -> Vec< (isize, isize, isize) >
{
    let mut mappings = Vec::< (isize, isize, isize) >::new();
    for line in lines {
        if line.contains(':') { continue; }
        if line.is_empty() { break; }
        let mapping = parse_nums(line);
        mappings.push((mapping[0], mapping[1], mapping[2]));
    }
    return mappings;
}

fn apply_mapping(values: &Vec< isize >, mapping: Vec< (isize, isize, isize) >) -> Vec< isize >
{
    let map_value = |x: &isize| {
        for (dst, src, len) in &mapping {
            if x >= src && *x < src + len {
                return dst + x - src
            }
        }
        return *x;
    };
    values.iter().map(map_value).collect()
}

fn part1(input: &str) -> isize
{
    let mut splits = input.split_terminator("\n\n");
    let mut seeds = get_seeds(splits.next().unwrap());
    splits.for_each(|split| {
        seeds = apply_mapping(&seeds, get_mapping(get_str_lines(split)));
    });
    return *seeds.iter().min().unwrap();
}

fn get_seed_ranges(line: &str) -> Vec< isize >
{
    let mut seeds = Vec::< isize >::new();
    for range in parse_nums(line.split(':').nth(1).unwrap()).chunks(2) {
        for i in 0..range[1] {
            seeds.push(range[0] + i);
        }
    }
    return seeds;
}

fn part2(input: &str) -> isize
{
    let mut splits = input.split_terminator("\n\n");
    let mut seeds = get_seed_ranges(splits.next().unwrap());
    splits.for_each(|split| {
        seeds = apply_mapping(&seeds, get_mapping(get_str_lines(split)));
    });
    return *seeds.iter().min().unwrap();
}

fn main() {
    println!("{}", part1(&get_file("inputs/day05.txt")));
    println!("{}", part2(&get_file("inputs/day05.txt")));
}
