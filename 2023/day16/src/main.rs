use std::{ fs::read_to_string, collections::HashSet, cmp };

fn get_map(file: &str) -> Vec< Vec< char > > {
    read_to_string(file).expect("invalid file")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum DIR {N, E, S, W}

fn move_beam(map: &Vec< Vec< char > >,
             pos: &(isize, isize),
             dir: DIR,
             history: &mut HashSet< (isize, isize, DIR) >
) -> HashSet< (usize, usize) > {
    if history.contains(&(pos.0, pos.1, dir.clone())) { return HashSet::< (usize, usize) >::new() }
    history.insert((pos.0, pos.1, dir.clone()));
    if pos.0 < 0 || pos.0 >= map.len() as isize || pos.1 < 0 || pos.1 >= map[0].len() as isize {
        return HashSet::< (usize, usize) >::new()
    };
    let (row, col) = (pos.0 as usize, pos.1 as usize);
    let mut set = match dir {
        DIR::N => match map[row][col] {
            '/'  => { move_beam(map, &(row as isize, col as isize + 1), DIR::E, history) },
            '\\' => { move_beam(map, &(row as isize, col as isize - 1), DIR::W, history) },
            '-'  => { move_beam(map, &(row as isize, col as isize - 1), DIR::W, history).union(
                      &move_beam(map, &(row as isize, col as isize + 1), DIR::E, history))
                      .map(|&x|x).collect()},
             _   => { move_beam(map, &(row as isize - 1, col as isize), dir, history)}
        },
        DIR::E => match map[row][col] {
            '/'  => { move_beam(map, &(row as isize - 1, col as isize), DIR::N, history) },
            '\\' => { move_beam(map, &(row as isize + 1, col as isize), DIR::S, history) },
            '|'  => { move_beam(map, &(row as isize - 1, col as isize), DIR::N, history).union(
                      &move_beam(map, &(row as isize + 1, col as isize), DIR::S, history))
                      .map(|&x|x).collect()},
             _   => { move_beam(map, &(row as isize, col as isize + 1), dir, history)}
        },
        DIR::S => match map[row][col] {
            '/'  => { move_beam(map, &(row as isize, col as isize - 1), DIR::W, history) },
            '\\' => { move_beam(map, &(row as isize, col as isize + 1), DIR::E, history) },
            '-'  => { move_beam(map, &(row as isize, col as isize - 1), DIR::W, history).union(
                      &move_beam(map, &(row as isize, col as isize + 1), DIR::E, history))
                      .map(|&x|x).collect()},
             _   => { move_beam(map, &(row as isize + 1, col as isize), dir, history)}
        },
        DIR::W => match map[row][col] {
            '/'  => { move_beam(map, &(row as isize + 1, col as isize), DIR::S, history) },
            '\\' => { move_beam(map, &(row as isize - 1, col as isize), DIR::N, history) },
            '|'  => { move_beam(map, &(row as isize - 1, col as isize), DIR::N, history).union(
                      &move_beam(map, &(row as isize + 1, col as isize), DIR::S, history))
                      .map(|&x|x).collect()},
             _   => { move_beam(map, &(row as isize, col as isize - 1), dir, history)}
        },
    };
    set.insert((row, col));
    return set;
}

fn part1(map: &Vec< Vec< char > >) -> usize {
    move_beam(map, &(0,0), DIR::E, &mut HashSet::< (isize, isize, DIR) >::new()).iter().count()
}

fn part2(map: &Vec< Vec< char > >) -> usize {
    let mut max = 0;
    for i in 0..map.len() {
        max = cmp::max(move_beam(map, &(i as isize,0), DIR::E, &mut HashSet::< (isize, isize, DIR) >::new()).iter().count(), max);
        max = cmp::max(move_beam(map, &(i as isize,map[0].len() as isize), DIR::W, &mut HashSet::< (isize, isize, DIR) >::new()).iter().count(), max);
    }
    for i in 0..map[0].len() {
        max = cmp::max(move_beam(map, &(0, i as isize), DIR::S, &mut HashSet::< (isize, isize, DIR) >::new()).iter().count(), max);
        max = cmp::max(move_beam(map, &(map.len() as isize,i as isize), DIR::N, &mut HashSet::< (isize, isize, DIR) >::new()).iter().count(), max);
    }
    return max
}

fn main() {
    println!("Part1: {}", part1(&get_map("inputs/day16.txt")));
    println!("Part2: {}", part2(&get_map("inputs/day16.txt")));
}
