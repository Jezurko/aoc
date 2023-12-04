use std::{ fs::read_to_string, collections::{ HashSet, VecDeque } };
use regex::{Regex};
use lazy_static::lazy_static;

fn get_file(file: &str) -> String { read_to_string(file).expect("invalid file") }
fn get_str_lines(input: &String) -> Vec< &str > { input.lines().collect() }

lazy_static!{ static ref re:Regex = Regex::new(r"[0-9]+").unwrap(); }

fn process_card(card: &str) -> usize
{
    let sides = card.split(':')
        .collect::< Vec< &str > >()[1]
        .split('|')
        .map(|x| re.find_iter(x))
        .map(|list| list
                        .map(|num| num.as_str().parse::< isize >().unwrap())
                        .collect::< HashSet< isize > >()
            ).collect::< Vec< HashSet< isize > > >();
    return sides[0].intersection(&sides[1]).count();
}

fn part1(cards: Vec< &str >) -> isize
{
    cards.iter()
         .map(|x| process_card(x) as u32)
         .filter(|&x| x > 0)
         .map(|x| 2_isize.pow(x - 1))
         .sum::< isize >()
}

fn part2(cards: Vec< &str >) -> isize
{
    let mut total = 0;
    let mut copies_q = VecDeque::< isize >::new();
    for card in cards {
        let copies = copies_q.pop_front().unwrap_or(1);
        total = total + copies;
        let won_cards = process_card(card);
        if won_cards > copies_q.len() {
            copies_q.resize(won_cards, 1);
        }
        for i in 0..won_cards {
            copies_q[i] = copies_q[i] + copies;
        }
    }
    return total;
}

fn main() {
    println!("{}",part1(get_str_lines(&get_file("inputs/day04.txt"))));
    println!("{}",part2(get_str_lines(&get_file("inputs/day04.txt"))));
}
