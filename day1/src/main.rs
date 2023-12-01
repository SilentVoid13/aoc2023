use std::time::Instant;

use color_eyre::eyre::{ContextCompat, Result};

fn part1(input: &str) -> Result<usize> {
    let mut sum = 0;
    for l in input.lines() {
        let mut it = l.chars().filter(|c| c.is_ascii_digit());
        let d1 = if let Some(v) = it.next() {
            v
        } else {
            continue;
        };
        let d2 = it.last().unwrap_or(d1);
        let val = (d1 as u8 - '0' as u8) as usize * 10 + (d2 as u8 - '0' as u8) as usize;
        sum += val;
    }
    Ok(sum)
}

fn part2(input: &str) -> Result<usize> {
    let mut sum = 0;

    for l in input.lines() {
        let mut values = vec![];
        let mut i = 0;
        while i < l.len() {
            let c = l.chars().skip(i).next().unwrap();
            if c.is_ascii_digit() {
                let v = c as u8 - '0' as u8;
                values.push(v as usize);
            }
            match &l[i..] {
                l if l.starts_with("one") => {
                    values.push(1);
                }
                l if l.starts_with("two") => {
                    values.push(2);
                }
                l if l.starts_with("three") => {
                    values.push(3);
                }
                l if l.starts_with("four") => {
                    values.push(4);
                }
                l if l.starts_with("five") => {
                    values.push(5);
                }
                l if l.starts_with("six") => {
                    values.push(6);
                }
                l if l.starts_with("seven") => {
                    values.push(7);
                }
                l if l.starts_with("eight") => {
                    values.push(8);
                }
                l if l.starts_with("nine") => {
                    values.push(9);
                }
                _ => {}
            }
            i += 1;
        }
        let mut it = values.iter();
        let d1 = it.next().wrap_err("no first digit")?;
        let d2 = it.last().unwrap_or(d1);
        let val = d1 * 10 + d2;
        sum += val;
    }
    Ok(sum)
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
