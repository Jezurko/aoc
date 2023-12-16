use std::{ fs::read_to_string, collections::VecDeque };
use itertools::Itertools;
use regex::{Regex};
use lazy_static::lazy_static;

fn get_file(file: &str) -> String { read_to_string(file).expect("invalid file") }
fn get_str_lines(input: &str) -> Vec< &str > { input.lines().collect() }

lazy_static!{ static ref NUM:Regex = Regex::new(r"(-)?[0-9]+").unwrap(); }

fn parse_nums(input: &str) -> VecDeque< isize >
{
    NUM.find_iter(input).map(|num| num.as_str().parse::< isize >().unwrap()).collect()
}

fn check_zeroes(line: &VecDeque< isize >) -> bool { line.iter().fold(true, |acc, &x| acc && x == 0) }

fn get_sequences(line: VecDeque< isize >) -> Vec< VecDeque< isize > >
{
    let mut sequences = Vec::< VecDeque< isize > >::new();
    sequences.push(line);

    while !check_zeroes(sequences.last().unwrap()) {
        let diffs =  sequences.last().unwrap().iter()
                                     .tuple_windows()
                                     .map(|(x, y): (&isize, &isize)| y - x).collect();
        sequences.push(diffs);
    }
    return sequences;
}

fn extrapolate(sequences: &mut Vec< VecDeque< isize > >)
{
    let (mut prev_first, mut prev_last) = (0, 0);
    for sequence in sequences.iter_mut().rev() {
        prev_first = sequence.front().unwrap_or(&0) - prev_first;
        prev_last = sequence.back().unwrap_or(&0) + prev_last;
        sequence.push_back(prev_last);
        sequence.push_front(prev_first);
    }
}

fn solve(lines: &Vec< &str >) -> (isize, isize)
{
    let (mut start_sum, mut end_sum) = (0, 0);
    for line in lines {
        let mut seqs = get_sequences(parse_nums(line));
        extrapolate(&mut seqs);
        start_sum = start_sum + seqs.first().unwrap().front().unwrap();
        end_sum = end_sum + seqs.first().unwrap().back().unwrap();
    }
    return (start_sum, end_sum);
}

fn main() {
    let (history, future) = solve(&get_str_lines(&get_file("inputs/day09.txt")));
    println!("part1: {}, part2: {}", future, history);
}
