use std::fs::read_to_string;
use std::cmp;

fn get_file(file: &str) -> String { read_to_string(file).expect("invalid file") }
fn get_char_lines(input: &str) -> Vec< Vec< char > > {
    input.lines().map(|x|x.chars().collect::< Vec< char > >()).collect()
}

fn manhattan((x1, y1): &(usize, usize), (x2, y2): &(usize, usize),
             empty_rows: &Vec< bool >, empty_cols: &Vec< bool >, scale: usize
) -> usize {
    x1.abs_diff(*x2)
    + empty_rows[cmp::min(*x1, *x2)..cmp::max(*x1, *x2)]
                .into_iter().fold(0, |acc, &x| if x {acc + scale - 1} else { acc })
    + y1.abs_diff(*y2)
    + empty_cols[cmp::min(*y1, *y2)..cmp::max(*y1, *y2)]
                .into_iter().fold(0, |acc, &x| if x {acc + scale - 1} else { acc })
}

fn is_empty_col(map: &Vec< Vec< char > >, y: usize) -> bool {
    (0..map.len()).fold(true, |acc, x| acc && map[x][y] == '.')
}

fn is_empty_row(map: &Vec< Vec< char > >, x: usize) -> bool {
    map[x].iter().fold(true, |acc, &x| acc && x == '.')
}

fn find_galaxies(map: &Vec< Vec< char > >) -> Vec< (usize, usize) > {
    let mut galaxies = Vec::< (usize, usize) >::new();
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] == '#' { galaxies.push((x, y)) };
        }
    }
    return galaxies;
}

fn solve(galaxies: &Vec< (usize, usize) >,
         empty_rows: &Vec< bool >, empty_cols: &Vec< bool >, scale: usize
) -> usize {
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            sum = sum + manhattan(&galaxies[i], &galaxies[j], &empty_rows, &empty_cols, scale);
        }
    }
    return sum;
}

fn main() {
    let map = get_char_lines(&get_file("inputs/day11.txt"));
    let galaxies = find_galaxies(&map);
    let empty_rows = (0..map.len()).map(|x| is_empty_row(&map, x)).collect::< Vec< bool > >();
    let empty_cols = (0..map[0].len()).map(|y| is_empty_col(&map, y)).collect::< Vec< bool > >();

    println!("Part1: {}", solve(&galaxies, &empty_rows, &empty_cols, 2));
    println!("Part2: {}", solve(&galaxies, &empty_rows, &empty_cols, 1000000));
}
