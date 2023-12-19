use std::time::Instant;

use color_eyre::eyre::Result;

fn part1(input: &str) -> Result<isize> {
    let mut sum = 0;
    for l in input.lines() {
        let mut vals: Vec<isize> = l
            .split(" ")
            .filter_map(|v| v.parse::<isize>().ok())
            .collect();
        let mut seqs = vec![vals.clone()];
        let mut zero = false;
        while !zero {
            vals = vals.windows(2).map(|w| w[1] - w[0]).collect();
            zero = vals.iter().all(|v| *v == 0);
            seqs.push(vals.clone());
        }
        let mut last = 0;
        for s in seqs.iter().rev() {
            let l = s.iter().last().unwrap();
            last = last + l;
        }
        sum += last;
    }
    Ok(sum)
}

fn part2(input: &str) -> Result<isize> {
    let mut sum = 0;
    for l in input.lines() {
        let mut vals: Vec<isize> = l
            .split(" ")
            .filter_map(|v| v.parse::<isize>().ok())
            .collect();
        let mut seqs = vec![vals.clone()];
        let mut zero = false;
        while !zero {
            vals = vals.windows(2).map(|w| w[1] - w[0]).collect();
            zero = vals.iter().all(|v| *v == 0);
            seqs.push(vals.clone());
        }
        let mut first = 0;
        for s in seqs.iter().rev() {
            let l = s.iter().next().unwrap();
            first = l - first;
        }
        sum += first;
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
