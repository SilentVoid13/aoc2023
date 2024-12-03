use std::{collections::HashMap, time::Instant};

use color_eyre::eyre::Result;

fn hash(s: &str) -> usize {
    let mut h: usize = 0;
    for c in s.as_bytes() {
        h += *c as usize;
        h *= 17;
        h %= 256;
    }
    h
}

fn part1(input: &str) -> Result<usize> {
    let mut sum = 0;
    for step in input.trim().split(",") {
        let h = hash(step);
        sum += h;
    }
    Ok(sum)
}

fn part2(input: &str) -> Result<usize> {
    let mut boxes: Vec<HashMap<&str, (usize, usize)>> = vec![HashMap::default(); 256];
    let mut id = 0;
    for step in input.trim().split(",") {
        let (label, value) = step.split_once(['-', '=']).unwrap();
        let boxx = &mut boxes[hash(label)];
        if let Ok(value) = value.parse::<usize>() {
            if let Some((prev_id, _)) = boxx.get(label) {
                boxx.insert(label, (*prev_id, value));
            } else {
                boxx.insert(label, (id, value));
                id += 1;
            }
        } else {
            boxx.remove(label);
        }
    }
    let mut sum = 0;
    for (i, boxx) in boxes.into_iter().enumerate() {
        let mut items = boxx.into_iter().collect::<Vec<_>>();
        items.sort_by(|a, b| a.1.0.cmp(&b.1.0));
        for (slot, (_, (_, value))) in items.into_iter().enumerate() {
            let v = value * (slot + 1) * (i + 1);
            sum += v;
        }
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
