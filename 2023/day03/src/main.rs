use std::fs::read_to_string;
use regex::{Regex};
use lazy_static::lazy_static;
use std::cmp;
use std::collections::HashSet;

fn get_file(file: &str) -> String { read_to_string(file).expect("invalid file") }

lazy_static!{ static ref re:Regex = Regex::new(r"[0-9]+").unwrap(); }

fn insert_surrounding_pos(lines: &Vec< &str >,
                          symbol_pos: usize,
                          nums: &mut HashSet::< (usize, usize, usize) >,
                          l: usize)
{
    let line = &lines[l];
    for num in re.find_iter(line) {
        let (start, end) = (num.start(), num.end().saturating_sub(1));
        if !num.is_empty() &&
            ((start >= symbol_pos.saturating_sub(1) && start <= symbol_pos + 1)
            || (end >= symbol_pos.saturating_sub(1) && end <= symbol_pos + 1))
        {
            nums.insert((l, num.start(), num.end()));
        }
    }
}

fn get_num(line: &str, start: usize, end: usize) -> isize
{
    line[start..end].parse::< isize >().unwrap_or(0)
}

fn part1(input: &str) -> isize
{
    let lines: Vec< &str > = input.lines().collect();
    let symbol_re = Regex::new(r"[^0-9\.]").unwrap();
    let mut nums = HashSet::< (usize, usize, usize) >::new();
    for i in 0..lines.len() {
        for matched in symbol_re.find_iter(lines[i]) {
            for j in i.saturating_sub(1)..=cmp::min(i + 1, lines.len() - 1) {
                insert_surrounding_pos(&lines, matched.start(), &mut nums, j);
            }
        }
    }
    return nums.iter()
               .map(|(l, x, y)| get_num(lines[*l], *x, *y))
               .sum::< isize >();
}

fn part2(input: &str) -> isize
{
    let mut sum = 0;
    let lines: Vec< &str > = input.lines().collect();
    for i in 0..lines.len() {
        for (j, _) in lines[i].match_indices('*'){
            let mut gear = HashSet::< (usize, usize, usize) >::new();
            for k in i.saturating_sub(1)..=cmp::min(i + 1, lines.len() - 1) {
                insert_surrounding_pos(&lines, j, &mut gear, k);
            }
            if gear.len() == 2 {
                sum = sum + gear.iter()
                                .map(|(l, x, y)| get_num(lines[*l], *x, *y))
                                .product::< isize >();
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
