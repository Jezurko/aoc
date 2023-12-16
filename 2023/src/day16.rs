use aoc::accessor::*;
use std::{ fs::read_to_string, collections::HashSet, cmp };
use map_macro::hash_set;

fn get_map(file: &str) -> Vec< Vec< char > > {
    read_to_string(file).expect("invalid file")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum DIR {N, E, S, W}

fn move_beam(map: &Vec< Vec< char > >,
             pos: (isize, isize),
             dir: DIR,
             history: &mut HashSet< (isize, isize, DIR) >
) -> HashSet< (isize, isize) > {
    let (row, col) = (pos.0, pos.1);

    if history.contains(&(row, col, dir.clone())) { return hash_set!{}; }
    history.insert((row, col, dir.clone()));

    if row < 0 || row >= map.len() as isize || col < 0 || col >= map[0].len() as isize {
        return hash_set!{};
    };

    let mut set = match dir {
        DIR::N => match map.at(row).at(col) {
            '/'  => { move_beam(map, (row, col + 1), DIR::E, history) },
            '\\' => { move_beam(map, (row, col - 1), DIR::W, history) },
            '-'  => { move_beam(map, (row, col - 1), DIR::W, history).union(
                      &move_beam(map, (row, col + 1), DIR::E, history))
                      .map(|&x|x).collect()},
             _   => { move_beam(map, (row - 1, col), dir, history)}
        },
        DIR::E => match map.at(row).at(col) {
            '/'  => { move_beam(map, (row - 1, col), DIR::N, history) },
            '\\' => { move_beam(map, (row + 1, col), DIR::S, history) },
            '|'  => { move_beam(map, (row - 1, col), DIR::N, history).union(
                      &move_beam(map, (row + 1, col), DIR::S, history))
                      .map(|&x|x).collect()},
             _   => { move_beam(map, (row, col + 1), dir, history)}
        },
        DIR::S => match map.at(row).at(col) {
            '/'  => { move_beam(map, (row, col - 1), DIR::W, history) },
            '\\' => { move_beam(map, (row, col + 1), DIR::E, history) },
            '-'  => { move_beam(map, (row, col - 1), DIR::W, history).union(
                      &move_beam(map, (row, col + 1), DIR::E, history))
                      .map(|&x|x).collect()},
             _   => { move_beam(map, (row + 1, col), dir, history)}
        },
        DIR::W => match map.at(row).at(col) {
            '/'  => { move_beam(map, (row + 1, col), DIR::S, history) },
            '\\' => { move_beam(map, (row - 1, col), DIR::N, history) },
            '|'  => { move_beam(map, (row - 1, col), DIR::N, history).union(
                      &move_beam(map, (row + 1, col), DIR::S, history))
                      .map(|&x|x).collect()},
             _   => { move_beam(map, (row, col - 1), dir, history)}
        },
    };
    set.insert((row, col));
    return set;
}

fn part1(map: &Vec< Vec< char > >) -> usize {
    move_beam(map, (0,0), DIR::E, &mut hash_set!{}).len()
}

fn part2(map: &Vec< Vec< char > >) -> usize {
    let mut max = 0;
    let height = map.len() as isize;
    let width = map[0].len() as isize;
    for i in 0..height {
        max = cmp::max(move_beam(map, (i, 0), DIR::E, &mut hash_set!{}).iter().count(), max);
        max = cmp::max(move_beam(map, (i, width), DIR::W, &mut hash_set!{}).iter().count(), max);
    }
    for i in 0..width {
        max = cmp::max(move_beam(map, (0, i), DIR::S, &mut hash_set!{}).iter().count(), max);
        max = cmp::max(move_beam(map, (height,i), DIR::N, &mut hash_set!{}).iter().count(), max);
    }
    return max
}

fn main() {
    println!("Part1: {}", part1(&get_map("inputs/day16.txt")));
    println!("Part2: {}", part2(&get_map("inputs/day16.txt")));
}
