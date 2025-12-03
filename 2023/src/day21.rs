use aoc::{ vecpp::*, input::* };
use std::collections::HashSet;

fn find_start(grid: &Vec< Vec< char > >) -> (isize, isize) {
    for i in 0..grid.len() as isize {
        for j in 0..grid[0].len() as isize {
            if *grid.at(i).at(j) == 'S' { return (i, j);}
        }
    }
    panic!();
}

fn reach_in(grid: &Vec< Vec< char > >, visited: &mut HashSet< ((isize, isize), isize) >, pos: (isize, isize), steps: isize) -> isize {
    if visited.contains(&(pos, steps)) { return 0 }
    visited.insert((pos, steps));
    if steps == 0 { return 1 }

    let (row, col) = (pos.0, pos.1);
    let mut succs = Vec::< (isize, isize) >::new();
    if grid.in_bounds(row - 1) { succs.push((row - 1, col)) }
    if grid.in_bounds(row + 1) { succs.push((row + 1, col)) }
    if grid[0].in_bounds(col - 1) { succs.push((row, col - 1)) }
    if grid[0].in_bounds(col + 1) { succs.push((row, col + 1)) }

    let res = succs.iter().filter(|(row, col)| { let field = *grid.at(*row).at(*col); return field == '.' || field == 'S'; }).map(|&pos| reach_in(grid, visited, pos, steps - 1)).sum();
    return res;
}

fn reach_in2(grid: &Vec< Vec< char > >, visited: &mut HashSet< ((isize, isize), isize) >, pos: (isize, isize), steps: isize) -> isize {
    if visited.contains(&(pos, steps)) { return 0 }
    visited.insert((pos, steps));
    if steps == 0 { return 1 }

    let (row, col) = (pos.0, pos.1);
    let mut succs = Vec::< (isize, isize) >::new();
    succs.push((row - 1, col));
    succs.push((row + 1, col));
    succs.push((row, col - 1));
    succs.push((row, col + 1));

    let height = grid.len() as isize;
    let width = grid[0].len() as isize;
    let res = succs.iter()
                   .filter(|(row, col)| {
                        let field = *grid.at(row.rem_euclid(height)).at(col.rem_euclid(width));
                        return field == '.' || field == 'S'; })
                   .map(|&pos| reach_in2(grid, visited, pos, steps - 1)).sum();
    return res;
}

fn main() {
    let grid = get_map("inputs/day21.txt");
    let start = find_start(&grid);
    let mut visited = HashSet::new();
    println!("{}", reach_in(&grid, &mut visited, start, 64));
    for i in (65..=500).step_by(131) {
        visited = HashSet::new();
        print!("{{ {}, {} }},", i, reach_in2(&grid, &mut visited, start, i));
    }
}
