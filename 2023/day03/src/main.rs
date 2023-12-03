use std::fs::read_to_string;
use regex::{Regex};
use lazy_static::lazy_static;
use std::cmp;
use std::collections::HashSet;

fn get_file(file: &str) -> String { read_to_string(file).expect("invalid file") }

lazy_static!{ static ref re:Regex = Regex::new(r"[0-9]+").unwrap(); }

fn insert_surrounding_pos(line: &str, symbol_pos: isize, nums: &mut HashSet::< (usize, usize, usize) >, l: isize)
{
    for num in re.find_iter(line) {
        let start = isize::try_from(num.start()).unwrap();
        let end = isize::try_from(num.end()).unwrap() - 1;
        if !num.is_empty() &&
            ((start >= symbol_pos - 1 && start <= symbol_pos + 1)
            || (end >= symbol_pos - 1 && end <= symbol_pos + 1))
        {
            nums.insert((usize::try_from(l).unwrap(), num.start(), num.end()));
        }
    }
}

fn get_num(line: &str, start: &usize, end: &usize) -> isize
{
    line[*start..*end].parse::< isize >().unwrap_or(0)
}

fn part1(input: &str) -> isize
{
    let lines: Vec< &str > = input.lines().collect();
    let symbol_re = Regex::new(r"[^0-9\.]").unwrap();
    let len_isize = isize::try_from(lines.len()).unwrap();
    let mut nums = HashSet::< (usize, usize, usize) >::new();
    for i in 0..lines.len() {
        let isize_i = isize::try_from(i).unwrap();
        for matched in symbol_re.find_iter(lines[i]) {
            let start = isize::try_from(matched.start()).unwrap();
            for j in cmp::max(0, isize_i - 1)..=cmp::min(isize_i + 1, len_isize - 1) {
                insert_surrounding_pos(lines[usize::try_from(j).unwrap()], start, &mut nums, j);
            }
        }
    }
    return nums.iter().map(|(l, x, y)| get_num(lines[*l], x, y)).sum::< isize >();
}

fn part2(input: &str) -> isize
{
    let mut sum = 0;
    let lines: Vec< &str > = input.lines().collect();
    for i in 0..lines.len() {
        for (j, _) in lines[i].match_indices('*'){
            let mut gear = HashSet::< (usize, usize, usize) >::new();
            let star_pos = isize::try_from(j).unwrap();
            let isize_i = isize::try_from(i).unwrap();
            let len_isize = isize::try_from(lines.len()).unwrap();
            for k in cmp::max(0, isize_i - 1)..=cmp::min(isize_i + 1, len_isize - 1) {
                insert_surrounding_pos(lines[usize::try_from(k).unwrap()], star_pos, &mut gear, k);
            }
            if gear.len() == 2 {
                sum = sum + gear.iter().map(|(l, x, y)| get_num(lines[*l], x, y)).product::< isize >();
            }
        }
    }
    return sum;
}

fn main()
{
    println!("{}", part1(get_file("inputs/day03.txt").as_str()));
    println!("{}", part2(get_file("inputs/day03.txt").as_str()));
}
