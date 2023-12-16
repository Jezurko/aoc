use std::{ fs::read_to_string, collections::HashMap };

fn get_map(file: &str) -> Vec< Vec< char > > {
    read_to_string(file).expect("invalid file").lines().map(|x| x.chars().collect()).collect()
}

fn move_up(map: &mut Vec< Vec< char > >) -> bool {
    let mut changed = false;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[x][y] == 'O' && map[x.saturating_sub(1)][y] == '.' {
                changed = true;
                map[x][y] = '.';
                map[x.saturating_sub(1)][y] = 'O';
            }
        }
    }
    return changed;
}

fn move_all_the_way(map: &mut Vec< Vec< char > >) { while move_up(map) {}; }

fn score_map(map: &Vec< Vec< char > >) -> usize {
    let mut sum = 0;
    for i in 0..map.len() {
        sum = sum + map[i].iter().filter(|&&x| x == 'O').count() * (map.len() - i)
    }
    return sum;
}

fn part1(map: &mut Vec< Vec< char > >) -> usize {
    move_all_the_way(map);
    return score_map(map);
}

fn rotate_all_dirs(map: &mut Vec< Vec< char > >) {
    for _ in 0..4 {
        move_all_the_way(map);
        *map = ((0..map[0].len()))
            .map(|i| map.iter().map(|inner| inner[i]).rev().collect())
            .collect::< Vec< Vec< char > > >();
    }
}

fn part2(map: &mut Vec< Vec< char > >) -> usize {
    let mut mem = HashMap::< Vec< Vec< char > >, usize>::new();
    for i in 0..1000000000{
        if let Some(r) = mem.get(map) {
            for _ in 0..((1000000000 - i) % (i - r)) {
                rotate_all_dirs(map);
            }
            return score_map(map);
        }
        mem.insert(map.clone(), i);
        rotate_all_dirs(map);
    }
    return score_map(map);
}

fn main() {
    println!("Part1: {}", part1(&mut get_map("inputs/day14.txt")));
    println!("Part2: {}", part2(&mut get_map("inputs/day14.txt")));
}
