use aoc::{ vecpp::*, input::* };
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Brick {
    x1: usize,
    y1: usize,
    z1: usize,
    x2: usize,
    y2: usize,
    z2: usize,
}

fn parse_brick(brick: &str) -> Brick {
    let parts = brick.split('~').collect::< Vec< &str > >();
    let coords0 = parts[0].split(',').map(|x| x.parse::< usize >().unwrap()).collect::< Vec< usize > >();
    let coords1 = parts[1].split(',').map(|x| x.parse::< usize >().unwrap()).collect::< Vec< usize > >();
    return Brick{x1: coords0[0], y1: coords0[1], z1: coords0[2],
                 x2: coords1[0], y2: coords1[1], z2: coords1[2] }
}

fn drop(bricks: &mut Vec< Brick >) -> usize {
    let mut tops = HashMap::< (usize, usize), usize >::new();
    bricks.iter_mut().map(|brick| drop_brick(brick, &mut tops)).filter(|&x| x).count()
}

fn drop_brick(brick: &mut Brick, tops: &mut HashMap< (usize, usize), usize >) -> bool {
    let top: usize;
    {
    let tops = &tops;
    top = (brick.x1..=brick.x2)
                .flat_map(|x| (brick.y1..=brick.y2)
                            .map(move |y| tops.get(&(x, y)).cloned().unwrap_or_default()))
                .max()
                .unwrap_or_default();
    }
    let diff = (brick.z1 - top).saturating_sub(1);
    brick.z1 = brick.z1 - diff;
    brick.z2 = brick.z2 - diff;
    (brick.x1..=brick.x2).for_each(|x| (brick.y1..=brick.y2)
                                        .for_each(|y| { tops.insert((x, y), brick.z2); }));
    if diff == 0 { return false; } else { return true; }
}


fn main() {
    let mut bricks = get_lines("inputs/day22.txt").iter()
                        .map(|line| parse_brick(line))
                        .collect::< Vec< Brick > >();
    bricks.sort_by(|b1, b2| b1.z1.cmp(&b2.z1));
    drop(&mut bricks);
    let mut desintegrable = 0;
    let mut total_falls = 0;
    for i in 0..bricks.len() {
        let mut removed = bricks[0..i].to_vec();
        removed.extend(bricks[i+1..bricks.len()].iter());
        let dropped = drop(&mut removed);
        if dropped == 0 {
            desintegrable += 1;
        }
        total_falls += dropped;
    }
    println!("Part1: {}", desintegrable);
    println!("Part2: {}", total_falls);
}
