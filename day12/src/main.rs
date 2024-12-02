use std::{collections::HashMap, time::Instant};

use color_eyre::eyre::Result;

fn dp(
    c_idx: usize,
    g_idx: usize,
    values: &[u8],
    groups: &[usize],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // memoization
    if let Some(&res) = cache.get(&(c_idx, g_idx)) {
        return res;
    }
    // we reached the end
    if c_idx >= values.len() {
        // check if it's a valid arrangement
        let r = (g_idx == groups.len()) as usize;
        cache.insert((c_idx, g_idx), r);
        return r;
    }

    let mut res = 0;

    // if it's a dot, we continue
    // if it's unknown, we try the dot possiblity
    if values[c_idx] == b'.' || values[c_idx] == b'?' {
        res += dp(c_idx + 1, g_idx, values, groups, cache);
    }

    // if it's a pound, we try to match the current group
    // if it's unknown, we try the pound possiblity
    if let Some(&g) = groups.get(g_idx) {
        let g_end = c_idx + g;
        if (g_end < values.len() && !values[c_idx..g_end].contains(&b'.') && values[g_end] != b'#')
            || (g_end == values.len() && !values[c_idx..g_end].contains(&b'.'))
        {
            res += dp(g_end + 1, g_idx + 1, values, groups, cache);
        }
    }

    cache.insert((c_idx, g_idx), res);
    res
}

fn part1(input: &str) -> Result<usize> {
    let mut total = 0;
    for line in input.lines() {
        let (values, groups) = line.split_once(" ").unwrap();
        let groups = groups
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
        let res = dp(0, 0, values.as_bytes(), &groups, &mut cache);
        total += res;
    }
    Ok(total)
}

fn part2(input: &str) -> Result<usize> {
    let mut total = 0;
    for line in input.lines() {
        let (values, groups) = line.split_once(" ").unwrap();
        let groups = groups
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut new_values = values.to_string();
        let mut new_groups = groups.clone();
        for _ in 0..4 {
            new_values.push('?');
            new_values.push_str(values);
            new_groups.extend_from_slice(&groups);
        }

        let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
        let res = dp(0, 0, new_values.as_bytes(), &new_groups, &mut cache);
        total += res;
    }
    Ok(total)
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
