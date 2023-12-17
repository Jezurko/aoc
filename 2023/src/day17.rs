use aoc::{ vecpp::*, input::* };
use priority_queue::PriorityQueue;
use std::{ collections::HashSet, cmp::Reverse };

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum DIR {U, R, D, L}

fn succs((row, col): (isize, isize),
         dir: DIR,
         steps: isize,
         (lower, upper): (isize, isize),
         map: &Vec< Vec< isize > >
) -> Vec< ((isize, isize), DIR) >
{
    let mut dirs: Vec< DIR > = if steps < upper { vec![dir] } else { vec![] };
    if steps >= lower {
        match dir {
            DIR::U | DIR::D => { dirs.extend(vec![DIR::L, DIR::R]) },
            DIR::R | DIR::L => { dirs.extend(vec![DIR::U, DIR::D]) },
        }
    }
    dirs.into_iter()
        .map(|dir| match dir {
                        DIR::U => {((row - 1, col), dir)},
                        DIR::R => {((row, col + 1), dir)},
                        DIR::D => {((row + 1, col), dir)},
                        DIR::L => {((row, col - 1), dir)},})
        .filter(|&((r, c), _)| { map.in_bounds(r) && map[0].in_bounds(c) })
        .collect()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct State {
    pos: (isize, isize),
    dir: DIR,
    steps: isize
}

fn cheapest_path(start: (isize, isize),
                 bounds: (isize, isize),
                 map: &Vec< Vec< isize > >,
) -> isize
{
    let mut solved: HashSet< State > = HashSet::new();
    let mut queue: PriorityQueue< State, Reverse< isize > > = PriorityQueue::new();
    queue.push(State{ pos: start, dir: DIR::R, steps: 0 }, Reverse(0));
    let end = (map.len() as isize - 1, map[0].len() as isize - 1);

    while let Some((s, Reverse(cost))) = queue.pop() {
        if solved.contains(&s) { continue; }
        solved.insert(s);
        let (pos, dir, steps) = (s.pos, s.dir, s.steps);
        if pos == end { return cost; }
        for (p, d) in succs(pos, dir, steps, bounds, map) {
            queue.push_increase(State{ pos: p,
                                       dir: d,
                                       steps: if d == dir { steps + 1 } else { 1 } },
                       Reverse(cost + map.at(p.0).at(p.1))
            );
        }
    }
    unreachable!();
}

fn main() {
    let map = get_digit_map("inputs/day17.txt");
    println!("Part1: {}", cheapest_path((0,0), (0, 3), &map));
    println!("Part2: {}", cheapest_path((0,0), (4, 10), &map));
}
