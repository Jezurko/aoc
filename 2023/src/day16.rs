use aoc::{ vecpp::*, input::* };
use std::{ collections::HashSet, cmp };
use map_macro::hash_set;

#[derive(PartialEq, Eq, Hash, Clone)]
enum DIR {N, E, S, W}

fn move_beam(map: &Vec< Vec< char > >,
             (row, col): (isize, isize),
             dir: DIR,
             history: &mut HashSet< (isize, isize, DIR) >
) -> HashSet< (isize, isize) > {
    if !map.in_bounds(row) || !map[0].in_bounds(col) { return hash_set!{}; };

    if history.contains(&(row, col, dir.clone())) { return hash_set!{}; }
    history.insert((row, col, dir.clone()));

    let mut set = match dir {
        DIR::N => match map.at(row).at(col) {
            '/'  => { move_beam(map, (row, col + 1), DIR::E, history) },
            '\\' => { move_beam(map, (row, col - 1), DIR::W, history) },
            '-'  => { let mut set = move_beam(map, (row, col - 1), DIR::W, history);
                      set.extend(&move_beam(map, (row, col + 1), DIR::E, history));
                      set }
             _   => { move_beam(map, (row - 1, col), dir, history)}
        },
        DIR::E => match map.at(row).at(col) {
            '/'  => { move_beam(map, (row - 1, col), DIR::N, history) },
            '\\' => { move_beam(map, (row + 1, col), DIR::S, history) },
            '|'  => { let mut set = move_beam(map, (row - 1, col), DIR::N, history);
                      set.extend(&move_beam(map, (row + 1, col), DIR::S, history));
                      set },
             _   => { move_beam(map, (row, col + 1), dir, history)}
        },
        DIR::S => match map.at(row).at(col) {
            '/'  => { move_beam(map, (row, col - 1), DIR::W, history) },
            '\\' => { move_beam(map, (row, col + 1), DIR::E, history) },
            '-'  => { let mut set = move_beam(map, (row, col - 1), DIR::W, history);
                      set.extend(&move_beam(map, (row, col + 1), DIR::E, history));
                      set},
             _   => { move_beam(map, (row + 1, col), dir, history)}
        },
        DIR::W => match map.at(row).at(col) {
            '/'  => { move_beam(map, (row + 1, col), DIR::S, history) },
            '\\' => { move_beam(map, (row - 1, col), DIR::N, history) },
            '|'  => { let mut set = move_beam(map, (row - 1, col), DIR::N, history);
                      set.extend(&move_beam(map, (row + 1, col), DIR::S, history));
                      set },
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
    let (height, width) = (map.len() as isize, map[0].len() as isize);
    for i in 0..height {
        max = cmp::max(move_beam(map, (i, 0), DIR::E, &mut hash_set!{}).len(), max);
        max = cmp::max(move_beam(map, (i, width), DIR::W, &mut hash_set!{}).len(), max);
    }
    for i in 0..width {
        max = cmp::max(move_beam(map, (0, i), DIR::S, &mut hash_set!{}).len(), max);
        max = cmp::max(move_beam(map, (height,i), DIR::N, &mut hash_set!{}).len(), max);
    }
    return max
}

fn main() {
    let map = get_map("inputs/day16.txt");
    println!("Part1: {}, Part2: {}", part1(&map), part2(&map));
}
