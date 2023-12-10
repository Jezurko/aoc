use std::fs::read_to_string;

fn get_file(file: &str) -> String { read_to_string(file).expect("invalid file") }
fn get_char_lines(input: &str) -> Vec< Vec< char > > {
    input.lines().map(|x|x.chars().collect::< Vec< char > >()).collect()
}

fn find_start(map: &Vec< Vec< char > >) -> (usize, usize)
{
    for i in 0..map.len() {
        match map[i].iter().position(|&x| x == 'S') {
            Some(x) => return (i, x),
            None    => ()
        };
    }
    panic!();
}

#[derive(Debug)]
enum FROM{
    north,
    east,
    south,
    west
}

fn start_succ(start: &(usize, usize), map: &Vec< Vec< char > >) -> ((usize, usize), FROM)
{
    let mut check = map[start.0][(start.1).saturating_sub(1)];
    if check == '-' || check == 'F' || check == 'L' { return ((start.0, start.1 - 1), FROM::east); }

    check = map[(start.0).saturating_add(1)][start.1];
    if check == '|' || check == 'F' || check == '7' { return ((start.0 + 1, start.1), FROM::north); }

    check = map[start.0][(start.1).saturating_add(1)];
    if check == '-' || check == '7' || check == 'J' { return ((start.0, start.1 + 1), FROM::west); }

    check = map[(start.0).saturating_sub(1)][start.1];
    if check == '|' || check == 'L' || check == 'J' { return ((start.0 - 1, start.1), FROM::south); }

    panic!();
}


fn find_next(pos: &(usize, usize), from: FROM, map: &Vec< Vec< char > >) -> ((usize, usize), FROM)
{
    match map[pos.0][pos.1] {
        '|' => match from {
            FROM::north => return ((pos.0 + 1, pos.1), FROM::north),
            FROM::south => return ((pos.0 - 1, pos.1), FROM::south),
            _           => ()
        }
        '-' => match from {
            FROM::west => return ((pos.0, pos.1 + 1), FROM::west),
            FROM::east => return ((pos.0, pos.1 - 1), FROM::east),
            _          => ()
        }
        'L' => match from {
            FROM::north => return ((pos.0, pos.1 + 1), FROM::west),
            FROM::east  => return ((pos.0 - 1, pos.1), FROM::south),
            _           => ()
        }
        'J' => match from {
            FROM::north => return ((pos.0, pos.1 - 1), FROM::east),
            FROM::west  => return ((pos.0 - 1, pos.1), FROM::south),
            _           => ()
        }
        '7' => match from {
            FROM::south => return ((pos.0, pos.1 - 1), FROM::east),
            FROM::west  => return ((pos.0 + 1, pos.1), FROM::north),
            _           => ()
        }
        'F' => match from {
            FROM::east => return ((pos.0 + 1, pos.1), FROM::north),
            FROM::south => return ((pos.0, pos.1 + 1), FROM::west),
            _           => ()
        }
        _  => panic!()
    }
    println!("{:?}, {}, {:?}", pos, map[pos.0][pos.1], from);
    panic!();
}

fn count_loop(map: &Vec< Vec< char > >) -> usize
{
    let start: (usize, usize) = find_start(map);
    let (mut pos, mut dir) = start_succ(&start, map);
    let mut steps = 1;
    while pos != start {
        (pos, dir) = find_next(&pos, dir, &map);
        steps = steps + 1;
    }
    return steps / 2;
}

fn fill_scaled(map: &mut Vec< Vec< char > >, pos: &(usize, usize), dir: &FROM)
{
    match &dir {
        FROM::north => {
            map[pos.0 * 2][pos.1 * 2] = 'X';
            map[pos.0 * 2 - 1][pos.1 * 2] = 'X';
        },
        FROM::east => {
            map[pos.0 * 2][pos.1 * 2] = 'X';
            map[pos.0 * 2][pos.1 * 2 + 1] = 'X';
        },
        FROM::south => {
            map[pos.0 * 2][pos.1 * 2] = 'X';
            map[pos.0 * 2 + 1][pos.1 * 2] = 'X';
        },
        FROM::west => {
            map[pos.0 * 2][pos.1 * 2] = 'X';
            map[pos.0 * 2][pos.1 * 2 - 1] = 'X';
        }
    }

}

fn trace_path_scaled(map: &Vec< Vec< char > >) -> Vec< Vec< char > >
{
    let start: (usize, usize) = find_start(map);
    let mut scaled_map = Vec::< Vec< char > >::new();
    scaled_map.resize(map.len() * 2, vec!['.'; map[0].len() * 2]);
    scaled_map[start.0 * 2][start.1 * 2] = 'X';
    let (mut pos, mut dir) = start_succ(&start, map);
    fill_scaled(&mut scaled_map, &pos, &dir);
    while pos != start {
        (pos, dir) = find_next(&pos, dir, &map);
        fill_scaled(&mut scaled_map, &pos, &dir);
    }
    return scaled_map;
}

fn fill_fields(map: &mut Vec< Vec< char > >, pos: &(usize, usize))
{
    if map[pos.0][pos.1] == '.' {
        map[pos.0][pos.1] = '#';
        fill_fields(map, &((pos.0).saturating_sub(1), pos.1));
        if pos.1 + 1 < map[0].len() { fill_fields(map, &((pos.0), pos.1 + 1)) };
        if pos.0 + 1 < map.len() { fill_fields(map, &((pos.0 + 1), pos.1)) };
        fill_fields(map, &(pos.0, (pos.1).saturating_sub(1)));
    }
}

fn fill_map(map: &mut Vec< Vec< char > >)
{
    for i in 0..map[0].len() {
        fill_fields(map, &(0, i));
        fill_fields(map, &(map.len() - 1, i));
    }
    for i in 0..map.len() {
        fill_fields(map, &(i, 0));
        fill_fields(map, &(i, map[0].len() - 1));
    }
}

fn unscale_map(map: &Vec< Vec< char > >) -> Vec< Vec< char > >
{
    let mut unscaled_map = Vec::< Vec< char > >::new();
    for x in (0..map.len()).step_by(2) {
        let mut line = Vec::< char >::new();
        for y in (0..map[0].len()).step_by(2) {
            line.push(map[x][y]);
        }
        unscaled_map.push(line);
    }
    return unscaled_map;
}

fn main() {
    let map = get_char_lines(&get_file("inputs/day10.txt"));
    println!("Part1: {}", count_loop(&map));
    let mut traced_map = trace_path_scaled(&map);
    fill_map(&mut traced_map);
    let unscaled_map = unscale_map(&traced_map);
    println!("Part2: {}",
        unscaled_map.iter().map(|line| line.iter().filter(|&&x| x == '.')
                           .count())
                           .sum::< usize >()
    );

}
