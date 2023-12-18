use aoc::{ input::* };
use regex::{ Regex };
use lazy_static::lazy_static;

fn instructions_basic(instructions: &Vec< String >) -> Vec< (isize, (isize, isize)) > {
    instructions.iter()
        .map(|line| {
            let splits = line.split(' ').collect::< Vec< &str > >();
            let dir = match splits[0] {
                "U" => (-1,0),
                "R" => (0,1),
                "D" => (1,0),
                "L" => (0,-1),
                 _  => panic!()
            };
            return (splits[1].parse::< isize >().unwrap(), dir);
        })
        .collect()
}

lazy_static!{ static ref HEX:Regex = Regex::new(r"#[0-9a-z]+").unwrap(); }

fn dir_from_char(c: char) -> (isize, isize) {
    match c {
        '0' => (0,1),
        '1' => (1,0),
        '2' => (0,-1),
        '3' => (-1,0),
         _  => panic!()
    }
}

fn instructions_hex(instructions: &Vec< String >) -> Vec< (isize, (isize, isize)) > {
    instructions.iter()
        .map(|line| HEX.find(line).unwrap().as_str())
        .map(|hex| (isize::from_str_radix(&hex[1..=5], 16).unwrap(),
                    dir_from_char(hex.chars().nth(6).unwrap())))
        .collect()
}

fn area(instructions: Vec< (isize, (isize, isize)) >) -> isize {
    let mut area = 0;
    let mut outline = 0;
    let first = (0,0);
    let mut curr = first;
    // For future reference: ddg shoelace theorem
    for (len, dir) in instructions {
        outline += len;
        let next = (curr.0 + len * dir.0, curr.1 + len * dir.1);
        area += curr.1 * next.0 - next.1 * curr.0;
        curr = next;
    }
    return (area + outline) / 2 + 1;
}

fn main() {
    let instructions = get_lines("inputs/day18.txt");
    println!("Part1: {}", area(instructions_basic(&instructions)));
    println!("Part2: {}", area(instructions_hex(&instructions)));
}
