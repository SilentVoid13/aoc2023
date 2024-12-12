mod point;

use color_eyre::eyre::Result;
use itertools::Itertools;
use point::Point;
use std::time::Instant;
use z3::{
    ast::{self, Ast},
    Config, Context, SatResult, Solver,
};

fn parse(input: &str) -> Vec<(Point, Point)> {
    let mut res = vec![];
    for line in input.lines() {
        let mut points = vec![];
        for s in line.split(" @ ") {
            let mut i = s.split(", ");
            let x = i.next().unwrap().trim().parse().unwrap();
            let y = i.next().unwrap().trim().parse().unwrap();
            let z = i.next().unwrap().trim().parse().unwrap();
            let p = Point::new(x, y, z);
            points.push(p);
        }
        res.push((points[0], points[1]));
    }
    res
}

fn lines_cross_within_bounds(
    p1: (f64, f64),
    v1: (f64, f64),
    p2: (f64, f64),
    v2: (f64, f64),
    bounds: (f64, f64),
) -> Option<(f64, f64)> {
    let (p1x, p1y) = p1;
    let (v1x, v1y) = v1;
    let (p2x, p2y) = p2;
    let (v2x, v2y) = v2;

    let (min_bound, max_bound) = bounds;
    let det = v1x * v2y - v1y * v2x;
    if det.abs() < 1e-10 {
        return None;
    }

    let num_t = (p2x - p1x) * v2y - (p2y - p1y) * v2x;
    let num_s = (p2x - p1x) * v1y - (p2y - p1y) * v1x;
    let t = num_t / det;
    let s = num_s / det;
    if t < 0.0 || s < 0.0 {
        return None;
    }

    let x = p1x + v1x * t;
    let y = p1y + v1y * t;
    if x >= min_bound && x <= max_bound && y >= min_bound && y <= max_bound {
        Some((x, y))
    } else {
        None
    }
}

fn part1(input: &str) -> Result<usize> {
    let input = parse(input);

    const MIN: f64 = 200000000000000.0;
    const MAX: f64 = 400000000000000.0;

    let mut count = 0;
    for v in input.iter().combinations(2) {
        let [v1, v2] = v[..] else {
            unreachable!();
        };

        if lines_cross_within_bounds(
            (v1.0.x as f64, v1.0.y as f64),
            (v1.1.x as f64, v1.1.y as f64),
            (v2.0.x as f64, v2.0.y as f64),
            (v2.1.x as f64, v2.1.y as f64),
            (MIN, MAX),
        )
        .is_some()
        {
            count += 1;
        }
    }
    Ok(count)
}

fn part2(input: &str) -> Result<i64> {
    let input = parse(input);

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let x = ast::Int::new_const(&ctx, "x");
    let y = ast::Int::new_const(&ctx, "y");
    let z = ast::Int::new_const(&ctx, "z");
    let vx = ast::Int::new_const(&ctx, "vx");
    let vy = ast::Int::new_const(&ctx, "vy");
    let vz = ast::Int::new_const(&ctx, "vz");

    // 5 is enough to find the correct solution
    for (i, val) in input.iter().enumerate().take(5) {
        let (p, v) = val;
        let ni = ast::Int::new_const(&ctx, format!("n{}", i).as_str());
        let zero = ast::Int::from_i64(&ctx, 0);
        solver.assert(&ni.gt(&zero));

        solver.assert(&((p.x + &ni * v.x) - (&x + &vx * &ni))._eq(&zero));
        solver.assert(&((p.y + &ni * v.y) - (&y + &vy * &ni))._eq(&zero));
        solver.assert(&((p.z + &ni * v.z) - (&z + &vz * &ni))._eq(&zero));
    }

    if solver.check() == SatResult::Sat {
        let model = solver.get_model().unwrap();
        let x = model.eval(&x, false).unwrap().as_i64().unwrap();
        let y = model.eval(&y, false).unwrap().as_i64().unwrap();
        let z = model.eval(&z, false).unwrap().as_i64().unwrap();
        return Ok(x + y + z);
    }
    panic!("no solution found!");
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("../input");

    let instant = Instant::now();
    let res = part1(input)?;
    let time = Instant::now() - instant;
    println!("[*] part 1: {} ({:?})", res, time);

    let instant = Instant::now();
    let res = part2(input)?;
    let time = Instant::now() - instant;
    println!("[*] part 2: {} ({:?})", res, time);

    Ok(())
}
