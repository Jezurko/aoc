use std::fs::read_to_string;
use std::cmp;
use itertools::Itertools;

fn get_file(file: &str) -> String { read_to_string(file).expect("invalid file") }
fn get_char_lines(input: &str) -> Vec< Vec< char > > {
    input.lines().map(|x|x.chars().collect::< Vec< char > >()).collect()
}

fn manhattan((x1, y1): &(usize, usize), (x2, y2): &(usize, usize),
             empty_rows: &Vec< bool >, empty_cols: &Vec< bool >, scale: usize
) -> usize {
    let (x_min, x_max) = (cmp::min(*x1, *x2), cmp::max(*x1, *x2));
    let (y_min, y_max) = (cmp::min(*y1, *y2), cmp::max(*y1, *y2));
    let empty_count = |line: &Vec< bool >, min: usize, max: usize| {
        line[min..max].iter().fold(0,  |acc, &x| if x { acc + 1 } else { acc })
    };
    x_max - x_min + y_max - y_min
    + (empty_count(empty_rows, x_min, x_max) + empty_count(empty_cols, y_min, y_max)) * (scale - 1)
}

fn is_empty_col(map: &Vec< Vec< char > >, y: usize) -> bool {
    (0..map.len()).fold(true, |acc, x| acc && map[x][y] == '.')
}

fn is_empty_row(map: &Vec< Vec< char > >, x: usize) -> bool {
    map[x].iter().fold(true, |acc, &x| acc && x == '.')
}

fn find_galaxies(map: &Vec< Vec< char > >) -> Vec< (usize, usize) > {
    map.iter().enumerate()
       .map(|(x, line)| line.iter()
                        .enumerate()
                        .filter(|&(_, &c)| c == '#')
                        .map(move |(y, _)| (x, y)))
       .flatten().collect::< Vec< (usize, usize) > >()
}

fn solve(galaxies: &Vec< (usize, usize) >,
         empty_rows: &Vec< bool >, empty_cols: &Vec< bool >, scale: usize
) -> usize {
    galaxies.iter().tuple_combinations()
            .map(|(galaxy_1, galaxy_2)| manhattan(&galaxy_1, &galaxy_2, &empty_rows, &empty_cols, scale))
            .sum()
}

fn main() {
    let map = get_char_lines(&get_file("inputs/day11.txt"));
    let galaxies = find_galaxies(&map);
    let empty_rows = (0..map.len()).map(|x| is_empty_row(&map, x)).collect::< Vec< bool > >();
    let empty_cols = (0..map[0].len()).map(|y| is_empty_col(&map, y)).collect::< Vec< bool > >();

    println!("Part1: {}", solve(&galaxies, &empty_rows, &empty_cols, 2));
    println!("Part2: {}", solve(&galaxies, &empty_rows, &empty_cols, 1000000));
}
