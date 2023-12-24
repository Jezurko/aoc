use aoc::{ vecpp::*, input::* };

fn slope(c: char, slopes: bool) -> Option< (isize, isize) > {
    if !slopes { return None }
    match c {
        '>' => Some((0, 1)),
        '<' => Some((0, -1)),
        '^' => Some((-1, 0)),
        'v' => Some((1, 0)),
         _  => None
    }
}

fn longest_path((row, col): (isize, isize), map: &Vec< Vec< char > >, visited: &mut Vec< Vec< bool > >, dist: usize, slopes: bool) -> usize {
    if row == (map.len() - 1) as isize {
        return dist;
    }
    let succs = if let Some((r_d, c_d)) = slope(*map.at(row).at(col), slopes) {
        vec![(row + r_d, col + c_d)]
    } else {
        vec![(0, 1), (0, -1), (-1, 0), (1, 0)].iter().map(|(r_d, c_d)| (row + r_d, col + c_d)).collect()
    };
    let mut result = 0;
    for (r_next, c_next) in succs {
         if map.in_bounds(r_next)
            && map[0].in_bounds(c_next)
            && *map.at(r_next).at(c_next) != '#'
            && !visited.at(r_next).at(c_next)
         {
                *visited.at_mut(r_next).at_mut(c_next) = true;
                result = result.max(longest_path((r_next, c_next), map, visited, dist + 1, slopes));
                *visited.at_mut(r_next).at_mut(c_next) = false;
         }
    }
    return result;
}

fn find_start(map: &Vec< Vec< char > >) -> (isize, isize) {
    let col = map[0].iter().position(|&c| c == '.').unwrap();
    return (0, col as isize);
}

fn main() {
    let map = get_map("inputs/day23.txt");
    let start = find_start(&map);
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    *visited.at_mut(start.0).at_mut(start.1) = true;
    println!("Part1: {}", longest_path(start, &map, &mut visited, 0, true));
    println!("Part2: {}", longest_path(start, &map, &mut visited, 0, false));
}
