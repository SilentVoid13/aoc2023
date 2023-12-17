use std::time::Instant;

use color_eyre::eyre::{Context, ContextCompat, Result};

fn parse_line1(l: &str) -> Result<Vec<usize>> {
    let (_, vals) = l.split_once(": ").wrap_err("invalid times")?;
    let vals = vals
        .trim()
        .split_whitespace()
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    Ok(vals)
}

fn parse_line2(l: &str) -> Result<usize> {
    let (_, vals) = l.split_once(": ").wrap_err("invalid times")?;
    let val = vals
        .trim()
        .split_whitespace()
        .filter(|v| !v.is_empty())
        .collect::<String>()
        .parse::<usize>()?;
    Ok(val)
}

fn part1(input: &str) -> Result<usize> {
    let mut it = input.lines();
    let times = parse_line1(it.next().unwrap()).wrap_err("invalid times")?;
    let dists = parse_line1(it.next().unwrap()).wrap_err("invalid distances")?;

    let mut total = 1;
    for (max_t, best_d) in times.iter().zip(dists.iter()) {
        let mut count = 0;
        for t in 0..*max_t {
            let d = t * (max_t - t);
            if d > *best_d { count += 1; }
        }
        total *= count;
    }
    Ok(total)
}

fn part2(input: &str) -> Result<usize> {
    let mut it = input.lines();
    let max_t = parse_line2(it.next().unwrap()).wrap_err("invalid times")?;
    let best_d = parse_line2(it.next().unwrap()).wrap_err("invalid distances")?;

    let mut count = 0;
    for t in 0..max_t {
        let d = t * (max_t - t);
        if d > best_d { count += 1; }
    }
    Ok(count)
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
