use std::fs::read_to_string;
use distance::levenshtein;

fn get_file(file: &str) -> String { read_to_string(file).expect("invalid file") }

fn find_horizontal(pattern: &Vec< Vec< char > >, smudged: bool) -> usize {
    for i in (1..pattern.len()).step_by(2) {
        if check_between(0, i, &pattern, smudged) {
            return (i / 2) + i % 2;
        }
    }
    for i in ((pattern.len() % 2)..pattern.len() - 1).step_by(2) {
        if check_between(i, pattern.len() - 1, &pattern, smudged) {
            return pattern.len() - (pattern.len() - i) / 2;
        }
    }
    return 0;
}

fn check_between(top: usize, bottom: usize, pattern: &Vec< Vec< char > >, smudged: bool) -> bool {
    let mut smudge = smudged;
    for i in 0..(bottom - top)/2 + 1 {
        let pattern_top = &*pattern[top + i].iter().collect::< String >();
        let pattern_bottom = &*pattern[bottom - i].iter().collect::< String >();
        if pattern_top != pattern_bottom {
            if smudge && levenshtein(pattern_top, pattern_bottom) == 1 {
                    smudge = false;
            } else {
                return false;
            }
        }
    }
    return true && !smudge;
}

fn find_vertical(pattern: &Vec< Vec< char > >, smudged: bool) -> usize {
    let transposed = (0..pattern[0].len())
        .map(|i| pattern.iter().map(|inner| inner[i]).collect())
        .collect::< Vec< Vec< char > > >();
    return find_horizontal(&transposed, smudged);
}

fn solve(patterns: &Vec< &str >, smudged: bool) -> usize {
    patterns.iter().map(|x| x.lines().map(|x| x.chars().collect()).collect())
                   .map(|pattern| find_horizontal(&pattern, smudged) * 100
                                  + find_vertical(&pattern, smudged))
                   .sum::< usize >()
}

fn main() {
    let input = get_file("inputs/day13.txt");
    let patterns: Vec< &str > = input.split("\n\n").collect();
    println!("Part1: {}, Part2: {}", solve(&patterns, false), solve(&patterns, true));
}
