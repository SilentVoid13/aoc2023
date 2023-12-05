use std::time::Instant;

use color_eyre::eyre::{eyre, Context, ContextCompat, Error, Result};

fn part1(input: &str) -> Result<usize> {
    Err(eyre!("todo"))
}

fn part2(input: &str) -> Result<usize> {
    Err(eyre!("todo"))
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
