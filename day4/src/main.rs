use std::{time::Instant, collections::HashSet};

use color_eyre::eyre::{ContextCompat, Result};

fn part1(input: &str) -> Result<usize> {
    let mut points = 0;
    for l in input.lines() {
        let (_, l) = l.split_once(": ").wrap_err("invalid line")?;
        let (winning, cur_nums) = l.split_once(" | ").wrap_err("invalid line")?;
        let winning: HashSet<usize> = winning.split(" ").filter_map(|n| n.trim().parse::<usize>().ok()).collect();
        let cur_nums: HashSet<usize> = cur_nums.split(" ").filter_map(|n| n.trim().parse::<usize>().ok()).collect();
        let inter = cur_nums.intersection(&winning);
        let i = inter.count() as u32;
        if i > 0 {
            points += 2_usize.pow(i - 1);
        }
    }
    Ok(points)
}

fn part2(input: &str) -> Result<usize> {
    let l = input.lines().count();
    let mut count = vec![1; l];

    for (idx, l) in input.lines().enumerate() {
        let (_, l) = l.split_once(": ").wrap_err("invalid line")?;
        let (winning, cur_nums) = l.split_once(" | ").wrap_err("invalid line")?;
        let winning: HashSet<usize> = winning.split(" ").filter_map(|n| n.trim().parse::<usize>().ok()).collect();
        let cur_nums: HashSet<usize> = cur_nums.split(" ").filter_map(|n| n.trim().parse::<usize>().ok()).collect();

        let c = count[idx];
        let inter = cur_nums.intersection(&winning);
        let val = inter.count();
        for i in 1..=val {
            count[idx+i] += c;
        }
    }
    Ok(count.iter().sum())
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
