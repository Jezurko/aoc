use std::{ fs::read_to_string, cmp::Ordering, collections::HashSet};

fn get_file(file: &str) -> String { read_to_string(file).expect("invalid file") }
fn get_str_lines(input: &str) -> Vec< &str > { input.lines().collect() }

const nine: u8 = '9' as u8;
fn parse_hand(hand: &str) -> (String, usize)
{
    let mut splits = hand.split(' ');
    let cards = splits.next().unwrap().chars().map(|x| match x {
                                    'T' => nine + 1,
                                    'J' => nine + 2,
                                    'Q' => nine + 3,
                                    'K' => nine + 4,
                                    'A' => nine + 5,
                                    _   => x as u8 } as char).collect::< String >();
    return (cards, splits.next().unwrap().parse::< usize >().unwrap());
}

fn remap_joker((cards, bid): (String, usize)) -> (String, usize) {
    (cards.replace((nine + 2) as char, "1"), bid)
}

fn hand_type(cards: &String) -> u8
{
    let unique = cards.chars().collect::< HashSet< char > >();
    let mut pairs = 0;
    let mut tripple = false;
    for u_card in unique
    {
        match cards.chars().filter(|&x| x == u_card).count() {
            5 => return 6,
            4 => return 5,
            3 => tripple = true,
            2 => pairs = pairs + 1,
            _ => {}

        };
    }
    if tripple { if pairs == 1 { return 4; } else { return 3; } };
    if pairs == 2 {return 2};
    if pairs == 1 {return 1};
    return 0;
}

fn compare_hands((cards_l, _): &(String, usize), (cards_r, _): &(String, usize)) -> Ordering
{
    let (type_l, type_r) = (hand_type(cards_l), hand_type(cards_r));
    if type_l == type_r { return cards_l.cmp(cards_r); }
    return type_l.cmp(&type_r);
}

fn j_count(cards: &String) -> usize { cards.chars().filter(|&x| x == '1').count() }

fn hand_type_j(cards: &String) -> u8
{
    let j = j_count(cards);
    let hand_t = hand_type(cards);
    match j {
        1 => match hand_t {0 => return hand_t + 1, 5 => return hand_t + 1, _ => return hand_t + 2},
        2 => match hand_t {2 => return hand_t + 3, _ => return hand_t + 2},
        3 => return hand_t + 2,
        4 => return 6,
        _ => return hand_t
    }

}

fn compare_hands_j((cards_l, _): &(String, usize), (cards_r, _): &(String, usize)) -> Ordering
{
    let (type_l, type_r) = (hand_type_j(cards_l), hand_type_j(cards_r));

    if type_l == type_r { return cards_l.cmp(cards_r); }
    return type_l.cmp(&type_r);
}

fn part1(lines: Vec< &str >) -> usize
{
    let mut hands = lines.iter().map(|x| parse_hand(x)).collect::< Vec< (String, usize) > >();
    hands.sort_by(compare_hands);
    hands.iter().enumerate().map(|(i, (_,bid))| (i + 1) * bid).sum()
}

fn part2(lines: Vec< &str >) -> usize
{
    let mut hands = lines.iter().map(|x| remap_joker(parse_hand(x)))
                                .collect::< Vec< (String, usize) > >();
    hands.sort_by(compare_hands_j);
    hands.iter().enumerate().map(|(i, (_,bid))| (i + 1) * bid).sum()
}

fn main() {
    println!("{}", part1(get_str_lines(&get_file("inputs/day07.txt"))));
    println!("{}", part2(get_str_lines(&get_file("inputs/day07.txt"))));
}
