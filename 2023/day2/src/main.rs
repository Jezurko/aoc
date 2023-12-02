use std::fs::read_to_string;
use regex::{Regex};
use std::cmp;

fn get_file(file: &str) -> String { read_to_string(file).expect("invalid file") }

fn get_pulls(game: &str) -> Vec< &str > { game.split(':').collect::< Vec< &str > >()[1].split(';').collect() }

enum Colour {red, green, blue}
fn get_colour(pull: &str, colour: Colour) -> isize
{
    let re: Regex = match colour {
        Colour::red   => Regex::new(r".* ([0-9]+) red.*").unwrap(),
        Colour::green => Regex::new(r".* ([0-9]+) green.*").unwrap(),
        Colour::blue  => Regex::new(r".* ([0-9]+) blue.*").unwrap(),
    };
    let res = re.captures(pull);
    let x = match res {
        Some(res) => res.get(1).unwrap().as_str().parse::< isize >().unwrap(),
        None      => 0
    };
    return x;
}

fn sum_games(file: &str) -> isize
{
    let content = get_file(file);
    let (mut sum, mut game) = (0, 0);
    'lines_c: for line in content.lines() {
        game += 1;
        for pull in get_pulls(line){
            if get_colour(pull, Colour::red) > 12 || get_colour(pull, Colour::green) > 13 || get_colour(pull, Colour::blue) > 14
            {
                continue 'lines_c;
            }
        }
        sum += game;
    }
    return sum;
}

fn find_minimal_power(line: &str) -> isize
{
    let (mut red, mut green, mut blue) = (0, 0, 0);
    for pull in get_pulls(line){
        red = cmp::max(red, get_colour(pull, Colour::red));
        green = cmp::max(green, get_colour(pull, Colour::green));
        blue = cmp::max(blue, get_colour(pull, Colour::blue));
    }
    return red * green * blue;
}

fn powers_sum(file: &str) -> isize { get_file(file).lines().map(|x|find_minimal_power(x)).sum() }

fn main()
{
    println!("{}", sum_games("inputs/day2.txt"));
    println!("{}", powers_sum("inputs/day2.txt"));
}
