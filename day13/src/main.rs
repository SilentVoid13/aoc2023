use std::{fmt::Display, time::Instant};

use color_eyre::eyre::Result;

#[derive(Debug, PartialEq, Eq)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

fn reflection<T: AsRef<str> + Display>(array: &[T], old_val: Option<usize>) -> Option<usize> {
    'col: for l1 in 0..array.len() - 1 {
        if array[l1].as_ref() == array[l1 + 1].as_ref() {
            for (i1, i2) in (0..=l1).rev().zip(l1 + 1..array.len()) {
                if array[i1].as_ref() != array[i2].as_ref() {
                    continue 'col;
                }
            }
            if let Some(old_val) = old_val {
                if l1 == old_val {
                    continue;
                }
            }
            return Some(l1);
        }
    }
    None
}

fn part1(input: &str) -> Result<(usize, Vec<Reflection>)> {
    let mut hor = 0;
    let mut ver = 0;
    let mut res = Vec::new();

    for pattern in input.split("\n\n") {
        let lines: Vec<&str> = pattern.lines().collect();
        let mut cols = Vec::new();
        for i in 0..lines[0].len() {
            let mut col = String::new();
            for line in &lines {
                col.push(line.as_bytes()[i] as char);
            }
            cols.push(col);
        }

        if let Some(v) = reflection(&cols, None) {
            ver += v + 1;
            res.push(Reflection::Vertical(v));
        }
        if let Some(h) = reflection(&lines, None) {
            hor += h + 1;
            res.push(Reflection::Horizontal(h));
        }
    }
    Ok((hor * 100 + ver, res))
}

fn part2(input: &str, old_ref: &[Reflection]) -> Result<usize> {
    let mut hor = 0;
    let mut ver = 0;

    for (pi, pattern) in input.split("\n\n").enumerate() {
        for fix in 0..pattern.len() {
            if pattern[fix..].starts_with("\n") {
                continue;
            }

            let mut pattern = pattern.as_bytes().to_vec();
            match pattern[fix] {
                b'.' => pattern[fix] = b'#',
                b'#' => pattern[fix] = b'.',
                _ => unreachable!(),
            }

            let pattern = unsafe { std::str::from_utf8_unchecked(&pattern) };
            let lines: Vec<&str> = pattern.lines().collect();
            let mut cols = Vec::new();
            for i in 0..lines[0].len() {
                let mut col = String::new();
                for line in &lines {
                    col.push(line.as_bytes()[i] as char);
                }
                cols.push(col);
            }

            let mut v1 = None;
            let mut v2 = None;

            match old_ref[pi] {
                Reflection::Vertical(v) => {
                    v1 = Some(v);
                }
                Reflection::Horizontal(h) => {
                    v2 = Some(h);
                }
            };
            if let Some(v) = reflection(&cols, v1) {
                ver += v + 1;
                break;
            }
            if let Some(h) = reflection(&lines, v2) {
                hor += h + 1;
                break;
            }
        }
    }
    Ok(hor * 100 + ver)
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("../input");

    let instant = Instant::now();
    let (res, old) = part1(input)?;
    let time = Instant::now() - instant;
    println!("[*] part 1: {} ({:?})", res, time);

    let instant = Instant::now();
    let res = part2(input, &old)?;
    let time = Instant::now() - instant;
    println!("[*] part 2: {} ({:?})", res, time);

    Ok(())
}
