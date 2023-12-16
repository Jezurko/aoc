use std::{ fs::read_to_string, collections::HashMap };
use aoc::{ vecpp::* };

fn get_file(file: &str) -> String { read_to_string(file).expect("invalid file") }

fn hash(sequence: &str) -> usize {
    sequence.bytes().fold(0, |acc: usize, x: u8| ((acc + x as usize) * 17) % 256)
}

fn part1(sequence: &str) -> usize {
    sequence[..sequence.len() - 1].split(',').map(|substr| hash(substr)).sum()
}

fn part2(sequence: &str) -> usize {
    let mut boxes = vec![HashMap::< &str, usize >::new(); 256];
    let mut box_ixs = vec![Vec::< &str >::new(); 256];

    sequence[..sequence.len() - 1].split(',').for_each(|cmd| {
        // Wtf, extracting these if bodies to lambdas somehow makes borrow checker
        // angr. I give up
        if cmd.contains('=') {
            let splits = cmd.split('=').collect::< Vec< &str > >();
            let label = splits[0];
            let hash = hash(label);
            let focal_length = splits[1].parse::< usize >().unwrap();
            if !boxes[hash].contains_key(label) {
                box_ixs[hash].push(label);
            }
            boxes[hash].insert(label, focal_length);
        }
        if cmd.contains('-') {
            let splits = cmd.split('-').collect::< Vec< &str > >();
            let label = splits[0];
            let hash = hash(label);
            if let Some(ix) = box_ixs[hash].find(&label) {
                box_ixs[hash].remove(ix);
                boxes[hash].remove(label);
            }
        }
    }
    );
    boxes.iter().enumerate()
         .map(|(box_ix, b)| box_ixs[box_ix].iter().enumerate()
                                           .map(move |(lens_ix, label)| {
                                               let focal_length = b.get(label).unwrap();
                                               (box_ix + 1) * (lens_ix + 1) * focal_length
                                           })
         )
        .flatten()
        .sum()
}

fn main() {
    println!("Part1: {}", part1(&get_file("inputs/day15.txt")));
    println!("Part2: {}", part2(&get_file("inputs/day15.txt")));
}
