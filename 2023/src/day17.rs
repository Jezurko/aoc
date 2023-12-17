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
        .filter(|&((c, r), _)| { map.in_bounds(c) && map[0].in_bounds(r) })
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
    let mut solved: HashSet< ((isize, isize), isize, DIR) > = HashSet::new();
    let mut queue: PriorityQueue< State, Reverse< isize > > = PriorityQueue::new();
    queue.push(State{ pos: start, dir: DIR::R, steps: 0 }, Reverse(0));
    let end = (map.len() as isize - 1, map[0].len() as isize - 1);

    while let Some((State{pos, dir, steps}, cost)) = queue.pop() {
        if solved.contains(&(pos, steps, dir)) { continue; }
        solved.insert((pos, steps, dir));
        if pos == end { return cost.0; }
        for (pos, d) in succs(pos, dir, steps, bounds, map) {
            queue.push(State{ pos: pos,
                              dir: d,
                              steps: if d == dir { steps + 1 } else { 1 }
                            },
                       Reverse(cost.0 + map.at(pos.0).at(pos.1))
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
