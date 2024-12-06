use std::time::Instant;

use aoc_utils::point::*;
use color_eyre::eyre::Result;

fn parse1(input: &str) -> (Vec<Point>, usize) {
    let mut res = Vec::with_capacity(1000);
    let mut start = Point::new(0, 0);
    res.push(start);
    let mut np = 0;
    for l in input.lines() {
        let mut s = l.split(" ");
        let dir = match s.next().unwrap() {
            "R" => RIGHT,
            "L" => LEFT,
            "U" => UP,
            "D" => DOWN,
            _ => unreachable!(),
        };
        let val = s.next().unwrap().parse::<i32>().unwrap();
        start += dir * val;
        np += val as usize;
        res.push(start);
    }
    (res, np)
}

fn parse2(input: &str) -> (Vec<Point>, usize) {
    let mut res = Vec::with_capacity(1000);
    let mut start = Point::new(0, 0);
    res.push(start);
    let mut np = 0;
    for l in input.lines() {
        let (_, hex) = l.split_once("(#").unwrap();
        let dir = match hex.as_bytes()[5] {
            b'0' => RIGHT,
            b'1' => DOWN,
            b'2' => LEFT,
            b'3' => UP,
            _ => unreachable!(),
        };
        let val = i32::from_str_radix(&hex[..5], 16).unwrap();
        start += dir * val;
        np += val as usize;
        res.push(start);
    }
    (res, np)
}

fn part1(input: &str) -> Result<usize> {
    let (points, np) = parse1(input);
    let mut sum: i64 = 0;

    // shoelace formula
    for win in points.windows(2) {
        let [p0, p1] = win else { unreachable!() };
        sum = sum
            .checked_add(((p0.y + p1.y) * (p0.x - p1.x)).into())
            .unwrap()
    }
    let area = 0.5 * sum.abs() as f64;

    // find number of interior points with pick's theorem
    let i = area - (np as f64 / 2.0) + 1.0;

    Ok(i as usize + np)
}

fn part2(input: &str) -> Result<usize> {
    let (points, np) = parse2(input);
    let mut sum: i64 = 0;

    // shoelace formula
    for win in points.windows(2) {
        let [p0, p1] = win else { unreachable!() };
        sum = sum
            .checked_add(((p0.y + p1.y) * (p0.x - p1.x)).into())
            .unwrap()
    }
    let area = 0.5 * sum.abs() as f64;

    // find number of interior points with pick's theorem
    let i = area - (np as f64 / 2.0) + 1.0;

    Ok(i as usize + np)
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
