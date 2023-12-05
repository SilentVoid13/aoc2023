use std::{collections::HashMap, time::Instant};

use color_eyre::eyre::{Context, ContextCompat, Result};

fn part1(input: &str) -> Result<usize> {
    let width = input.lines().count();
    let height = input.lines().next().wrap_err("invalid input")?.len();
    let input: String = input.lines().collect();
    let mut cur_num = String::new();
    let mut sum: usize = 0;
    let mut is_part = false;

    for y in 0..height {
        for x in 0..width {
            let i = y * height + x;
            let c = input.as_bytes()[i];
            if c.is_ascii_digit() {
                cur_num.push(c as char);
                if is_part {
                    continue;
                }
                let y_min = y.checked_sub(1).unwrap_or(0);
                let y_max = if y + 1 >= height { y } else { y + 1 };
                let x_min = x.checked_sub(1).unwrap_or(0);
                let x_max = if x + 1 >= width { x } else { x + 1 };

                // check neighbours
                for y1 in y_min..=y_max {
                    for x1 in x_min..=x_max {
                        let i = y1 * height + x1;
                        let c = input.as_bytes()[i];
                        if !c.is_ascii_digit() && c != b'.' {
                            is_part = true;
                        }
                    }
                }
            } else {
                if is_part {
                    let num: usize = cur_num.parse().wrap_err("invalid number")?;
                    sum += num;
                    is_part = false;
                }
                cur_num.clear();
            }
        }
    }
    Ok(sum)
}

fn part2(input: &str) -> Result<usize> {
    let width = input.lines().count();
    let height = input.lines().next().wrap_err("invalid input")?.len();
    let input: String = input.lines().collect();
    let mut cur_num = String::new();
    let mut cur_pos = (0, 0);
    let mut is_part = false;
    let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            let i = y * height + x;
            let c = input.as_bytes()[i];
            if c.is_ascii_digit() {
                cur_num.push(c as char);
                let y_min = y.checked_sub(1).unwrap_or(0);
                let y_max = if y + 1 >= height { y } else { y + 1 };
                let x_min = x.checked_sub(1).unwrap_or(0);
                let x_max = if x + 1 >= width { x } else { x + 1 };

                // check neighbours
                for y1 in y_min..=y_max {
                    for x1 in x_min..=x_max {
                        let i = y1 * height + x1;
                        let c = input.as_bytes()[i];
                        if c == b'*' {
                            is_part = true;
                            cur_pos = (x1, y1);
                        }
                    }
                }
            } else {
                if is_part {
                    let num: usize = cur_num.parse().wrap_err("invalid number")?;
                    is_part = false;
                    gears
                        .entry(cur_pos)
                        .and_modify(|v| v.push(num))
                        .or_insert(vec![num]);
                }
                cur_num.clear();
            }
        }
    }

    let mut sum: usize = 0;
    for v in gears.values() {
        if v.len() == 2 {
            sum += v[0] * v[1];
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
