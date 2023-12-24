use aoc::{ vecpp::*, input::* };
use z3::ast::Ast;

struct Hailstone {
    x: i64,
    y: i64,
    z: i64,
    x_v: i64,
    y_v: i64,
    z_v: i64
}

fn parse_hailstone(line: &str) -> Hailstone {
    let h = line.split(" @ ")
                      .map(|x| x.split(", "))
                      .flatten()
                      .map(|x| x.parse::< i64 >().unwrap()).collect::< Vec< i64 > >();
    return Hailstone{ x: h[0], y: h[1], z: h[2], x_v: h[3], y_v: h[4], z_v: h[5] };

}

fn intersect_in(h1: &Hailstone, h2: &Hailstone, min: i128, max: i128) -> bool {
    let (a1, b1, c1): (i128, i128, i128) = (h1.y_v as i128, -h1.x_v as i128, (h1.x_v * h1.y - h1.y_v * h1.x) as i128);
    let (a2, b2, c2): (i128, i128, i128) = (h2.y_v as i128, -h2.x_v as i128, (h2.x_v * h2.y - h2.y_v * h2.x) as i128);
    let divisor = a1 * b2 - a2 * b1;
    if divisor == 0 {
        return false;
    }
    let x = (c2 * b1 - c1 * b2) / divisor;
    let y = (a2 * c1 - a1 * c2) / divisor;
    if (x as i64 - h1.x).signum() != h1.x_v.signum() || (x as i64 - h2.x).signum() != h2.x_v.signum()
        || (y as i64 - h1.y).signum() != h1.y_v.signum() || (y as i64 - h2.y).signum() != h2.y_v.signum()
    {
        return false;
    }
    return min <= x && x <= max && min <= y && y <= max;
}

fn part2(hailstones: &Vec< Hailstone >) -> String {
    let config = z3::Config::new();
    let ctx = z3::Context::new(&config);
    let solver = z3::Solver::new(&ctx);

    let x = z3::ast::Real::new_const(&ctx, "x");
    let y = z3::ast::Real::new_const(&ctx, "y");
    let z = z3::ast::Real::new_const(&ctx, "z");

    let x_v = z3::ast::Real::new_const(&ctx, "x_v");
    let y_v = z3::ast::Real::new_const(&ctx, "y_v");
    let z_v = z3::ast::Real::new_const(&ctx, "z_v");

    for (i, h) in hailstones[..3].iter().enumerate() {
        let x1_i = z3::ast::Int::from_i64(&ctx, h.x);
        let y1_i = z3::ast::Int::from_i64(&ctx, h.y);
        let z1_i = z3::ast::Int::from_i64(&ctx, h.z);

        let x1 = z3::ast::Real::from_int(&x1_i);
        let y1 = z3::ast::Real::from_int(&y1_i);
        let z1 = z3::ast::Real::from_int(&z1_i);

        let x1_v = z3::ast::Real::from_real(&ctx, h.x_v as i32, 1);
        let y1_v = z3::ast::Real::from_real(&ctx, h.y_v as i32, 1);
        let z1_v = z3::ast::Real::from_real(&ctx, h.z_v as i32, 1);

        let time = z3::ast::Real::new_const(&ctx, i.to_string());

        solver.assert(&time.gt(&z3::ast::Real::from_real(&ctx, 0, 1)));

        solver.assert(&(&x + &x_v * &time)._eq(&(&x1 + &x1_v * &time)));
        solver.assert(&(&y + &y_v * &time)._eq(&(&y1 + &y1_v * &time)));
        solver.assert(&(&z + &z_v * &time)._eq(&(&z1 + &z1_v * &time)));
    }
    solver.check();
    let model = solver.get_model().unwrap();
    return model.eval(&(&x + &y + &z), true).unwrap().to_string();
}

fn main() {
    let hailstones: Vec< Hailstone > = get_lines("inputs/day24.txt").iter().map(|x| parse_hailstone(x)).collect();
    let mut intersections = 0;
    for i in 0..hailstones.len() {
        for j in i + 1.. hailstones.len() {
            if intersect_in(&hailstones[i], &hailstones[j], 200000000000000, 400000000000000) { intersections += 1; }
        }
    }
    println!("Part1: {}", intersections);
    println!("Part2: {}", part2(&hailstones));
}
