use aoc::input::*;
use std::collections::{ HashMap, VecDeque };
use num::integer::lcm;

struct Broadcaster {
    targets: Vec< String >
}

struct FlipFlop {
    state: bool,
    targets: Vec< String >
}

struct Conjunction {
    inputs: HashMap< String, bool >,
    targets: Vec< String >
}

fn parse_broadcaster(input: &Vec< String >) -> Broadcaster {
    for line in input {
        if line.starts_with("broadcaster") {
            let targets = line.split("-> ").nth(1).unwrap();
            return Broadcaster{ targets: targets.split(", ").map(|x| x.to_string()).collect() };
        }
    }
    panic!();
}

fn parse_flipflops(input: &Vec< String >) -> HashMap< String, FlipFlop > {
    let mut flipflops = HashMap::new();
    for line in input {
        if !line.starts_with("%") { continue; }

        let splits = line.split(" -> ").collect::< Vec< &str > >();
        let name = splits[0][1..].to_string();
        let targets = splits[1].split(", ").map(|x| x.to_string()).collect::< Vec< String > >();
        flipflops.insert(name, FlipFlop{state: false, targets});
    }
    return flipflops;
}

fn parse_conjunctions(input: &Vec< String >, flipflops: &HashMap< String, FlipFlop >) -> HashMap< String, Conjunction > {
    let mut conjunctions = HashMap::new();
    for line in input {
        if !line.starts_with("&") { continue; }

        let splits = line.split(" -> ").collect::< Vec< &str > >();
        let name = splits[0][1..].to_string();
        let targets = splits[1].split(", ").map(|x| x.to_string()).collect::< Vec< String > >();
        let mut inputs = HashMap::< String, bool >::new();
        for (ffname, ff) in flipflops.iter() {
            if ff.targets.contains(&name) {inputs.insert(ffname.clone(), false);}
        }
        conjunctions.insert(name, Conjunction{ inputs, targets });
    }

    let mut cj_ins = Vec::< (String, String) >::new();
    for (name, cj) in conjunctions.iter() {
        for trg in cj.targets.iter() {
            cj_ins.push((name.to_string(), trg.to_string()));
        }
    }
    for (name, trg) in cj_ins {
        if let Some(cj) = conjunctions.get_mut(&trg) {
            cj.inputs.insert(name, false);
        }
    }
    return conjunctions;
}

fn run_flipflop(name: &String, signal: bool, flipflop: &mut FlipFlop) -> Option< VecDeque< (bool, String, String) > > {
    if signal {return None;}
    flipflop.state = !flipflop.state;
    let queue = flipflop.targets.iter().map(|trg| (flipflop.state, trg.clone(), name.clone())).collect();
    return Some(queue);
}

fn run_conjunction(name: &String, signal: bool, source: &String, conjunction: &mut Conjunction) -> VecDeque< (bool, String, String) > {
    conjunction.inputs.entry(source.to_string()).and_modify(|x| *x = signal);
    let out = !conjunction.inputs.values().fold(true, |acc, &x| acc && x);
    return conjunction.targets.iter().map(|trg| (out, trg.clone(), name.clone())).collect();
}

fn solve(broadcaster: &Broadcaster, flipflops: &mut HashMap< String, FlipFlop>, conjunctions: &mut HashMap< String, Conjunction >) {
    let mut queue = VecDeque::< (bool, String, String) >::new();
    let mut i: usize = 0;
    let (mut low, mut high) = (0, 0);
    let mut rx_in = conjunctions.iter()
                                .filter(|(_, cj)| cj.targets.contains(&"kc".to_string()))
                                .map(|(name, _)| (name.to_string(), 0))
                                .collect::< HashMap< String, usize > >();
    loop {
        queue.extend(broadcaster.targets.iter().map(|trg| (false, trg.clone(), "broadcaster".to_string())));
        if i == 1000 {
            println!("Part1: {}", low * high);
        }
        i += 1;
        low += 1;

        while let Some((signal, target, source)) = queue.pop_front() {
            if signal { high += 1; } else { low += 1; }
            if signal {
                if let Some(iter) = rx_in.get_mut(&source) {
                    *iter = i;
                }
            }
            if let Some(ff) = flipflops.get_mut(&target) {
                if let Some(q) = run_flipflop(&target, signal, ff) {
                    queue.extend(q);
                }
            }
            if let Some(cj) = conjunctions.get_mut(&target) {
                queue.extend(run_conjunction(&target, signal, &source, cj));
            }
        }
        if rx_in.iter().fold(true, |acc, (_, x)| acc && *x > 0) {
            println!("Part2: {}", rx_in.values().fold(1, |acc, x| lcm(acc, *x)));
            return;
        }
    }
}

fn main() {
    let lines = get_lines("inputs/day20.txt");
    let broadcaster = parse_broadcaster(&lines);
    let mut ffs = parse_flipflops(&lines);
    let mut cjs = parse_conjunctions(&lines, &ffs);
    solve(&broadcaster, &mut ffs, &mut cjs);
}
