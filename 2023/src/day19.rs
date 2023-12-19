use aoc::{ input::* };
use regex::{ Regex };
use lazy_static::lazy_static;
use std::collections::{ HashMap, VecDeque };

lazy_static!{ static ref WF:Regex = Regex::new(r"([a-z]+)\{(.*)\}").unwrap(); }
lazy_static!{ static ref PART:Regex = Regex::new(r"\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\}").unwrap(); }

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize
}

fn parse_workflows(workflows: Vec< &str >) -> HashMap< String, String > {
    workflows.iter().map(|wf| WF.captures(wf).unwrap())
                    .map(|captures| (captures[1].to_string(), captures[2].to_string()))
                    .collect()
}

fn run_worklfow(part: &Part, wf: &str) -> String {
    let conds = wf.split(',');
    for cond in conds {
        if !cond.contains(':') {
            return cond.to_string();
        }
        let splits = cond.split(':').collect::< Vec< &str > >();
        let val = match splits[0].chars().nth(0).unwrap() {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
             _ => panic!()
        };
        let cmp = splits[0][2..].parse::< usize >().unwrap();
        if splits[0].chars().nth(1).unwrap() == '<' {
            if val < cmp {
                return splits[1].to_string();
            }
        } else {
            if val > cmp {
                return splits[1].to_string();
            }
        }
    };
    panic!();
}

fn parse_parts(parts: Vec< &str >) -> Vec< Part > {
    parts.iter().map(|x| { let values = PART.captures(x).unwrap();
                           return Part{x: values[1].parse().unwrap(),
                                       m: values[2].parse().unwrap(),
                                       a: values[3].parse().unwrap(),
                                       s: values[4].parse().unwrap() } })
                .collect()
}

fn part1(parts: &Vec< Part >, wfs: &HashMap< String, String >) -> usize {
    let mut sum = 0;
    for part in parts {
        let mut wf: String = "in".to_string();
        while wf != "A" && wf != "R" {
            wf = run_worklfow(&part, wfs.get(&wf).unwrap());
        }
        if wf == "A" {
            sum += part.x + part.m + part.a + part.s;
        }
    }
    return sum;
}

#[derive(Copy, Clone)]
struct Part_interval {
    x_min: usize,
    x_max: usize,
    m_min: usize,
    m_max: usize,
    a_min: usize,
    a_max: usize,
    s_min: usize,
    s_max: usize,
}

fn restrict_part(mut part: Part_interval, wf: &String) -> VecDeque< (Part_interval, String) > {
    let mut restricted = VecDeque::new();
    let conds = wf.split(',');
    for cond in conds {
        if !cond.contains(':') {
            restricted.push_back((part, cond.to_string()));
            continue;
        }
        let splits = cond.split(':').collect::< Vec< &str > >();
        let cmp = splits[0][2..].parse::< usize >().unwrap();
        let mut new_part = part;
        if splits[0].chars().nth(1).unwrap() == '<' {
            match splits[0].chars().nth(0).unwrap() {
                'x' => {new_part.x_max = cmp - 1; part.x_min = cmp},
                'm' => {new_part.m_max = cmp - 1; part.m_min = cmp},
                'a' => {new_part.a_max = cmp - 1; part.a_min = cmp},
                's' => {new_part.s_max = cmp - 1; part.s_min = cmp},
                 _ => panic!()
            };
            restricted.push_back((new_part, splits[1].to_string()));
        } else {
            match splits[0].chars().nth(0).unwrap() {
                'x' => {new_part.x_min = cmp + 1; part.x_max = cmp},
                'm' => {new_part.m_min = cmp + 1; part.m_max = cmp},
                'a' => {new_part.a_min = cmp + 1; part.a_max = cmp},
                's' => {new_part.s_min = cmp + 1; part.s_max = cmp},
                 _ => panic!()
            };
            restricted.push_back((new_part, splits[1].to_string()));
        }
    };
    return restricted;
}

fn part2(wfs: &HashMap< String, String >) -> usize {
    let part = Part_interval{x_min:1,
                             x_max:4000,
                             m_min:1,
                             m_max:4000,
                             a_min:1,
                             a_max:4000,
                             s_min:1,
                             s_max:4000,
    };
    let mut queue = VecDeque::< (Part_interval, String) >::new();
    queue.push_back((part, "in".to_string()));
    let mut sum = 0;
    while let Some((p, wf)) = queue.pop_front() {
        if wf == "A" {sum += (p.x_max - p.x_min + 1) *
                             (p.m_max - p.m_min + 1) *
                             (p.a_max - p.a_min + 1) *
                             (p.s_max - p.s_min + 1);}
        if wf == "R" || wf == "A" {continue;}
        queue.extend(restrict_part(p, wfs.get(&wf).unwrap()));
    }
    return sum;
}

fn main() {
    let input = get_file("inputs/day19.txt");
    let splits = input.split("\n\n").collect::< Vec< &str > >();
    let wfs = parse_workflows(splits[0].lines().collect::< Vec< &str > >());
    let parts = parse_parts(splits[1].lines().collect::< Vec< &str > >());
    println!("Part1: {}", part1(&parts, &wfs));
    println!("Part2: {}", part2(&wfs));
}
