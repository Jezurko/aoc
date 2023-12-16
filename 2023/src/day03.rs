use std::{fs::read_to_string, collections::HashSet, cmp};
use regex::{Regex};
use lazy_static::lazy_static;

fn get_file(file: &str) -> String { read_to_string(file).expect("invalid file") }
fn get_str_lines(input: &String) -> Vec< &str > { input.lines().collect() }

lazy_static!{ static ref re:Regex = Regex::new(r"[0-9]+").unwrap(); }

fn surrounding_pos(lines: &Vec< &str >,
                          symbol_pos: usize,
                          l: usize)
    -> HashSet::< (usize, usize, usize) >
{
    let mut nums = HashSet::< (usize, usize, usize) >::new();
    for num in re.find_iter(&lines[l]) {
        let (start, end) = (num.start(), num.end().saturating_sub(1));
        if !num.is_empty() &&
            ((start >= symbol_pos.saturating_sub(1) && start <= symbol_pos + 1)
            || (end >= symbol_pos.saturating_sub(1) && end <= symbol_pos + 1))
        {
            nums.insert((l, num.start(), num.end()));
        }
    }
    return nums;
}

fn process_symbol(lines: &Vec< &str >, line: usize, col: usize) -> HashSet< (usize, usize, usize) >
{
    let mut nums = HashSet::< (usize, usize, usize) >::new();
    for i in line.saturating_sub(1)..=cmp::min(line + 1, lines.len() - 1) {
        nums.extend(surrounding_pos(&lines, col, i));
    }
    return nums;
}

fn get_num(line: &str, start: usize, end: usize) -> isize
{
    line[start..end].parse::< isize >().unwrap_or(0)
}

fn part1(lines: Vec< &str >) -> isize
{
    let symbol_re = Regex::new(r"[^0-9\.]").unwrap();
    let mut nums = HashSet::< (usize, usize, usize) >::new();
    for (i, line) in lines.iter().enumerate() {
        for matched in symbol_re.find_iter(line) {
            nums.extend(process_symbol(&lines, i, matched.start()));
        }
    }
    return nums.iter()
               .map(|(l, x, y)| get_num(lines[*l], *x, *y))
               .sum::< isize >();
}

fn part2(lines: Vec< &str >) -> isize
{
    let mut sum = 0;
    for (i, line) in lines.iter().enumerate() {
        for (j, _) in line.match_indices('*') {
            let gear = process_symbol(&lines, i, j);
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
    println!("{}", part1(get_str_lines(&get_file("inputs/day03.txt"))));
    println!("{}", part2(get_str_lines(&get_file("inputs/day03.txt"))));
}
