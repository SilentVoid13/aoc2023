use std::{collections::HashMap, ops::Range, time::Instant};

use color_eyre::eyre::Result;

const MIN: usize = 1;
const MAX: usize = 4000;

fn parse(
    input: &str,
) -> Result<(
    HashMap<String, Vec<(Option<u8>, Range<usize>, String)>>,
    Vec<Vec<(u8, usize)>>,
)> {
    let (workflows, rules) = input.split_once("\n\n").unwrap();

    let mut res = HashMap::new();
    for l in workflows.lines() {
        let (name, rest) = l.split_once("{").unwrap();
        let (rest, _) = rest.split_once("}").unwrap();

        let mut conds = vec![];
        for s in rest.split(",") {
            let Some((val, dest)) = s.split_once(":") else {
                let s = s.to_string();
                let c = MIN..MAX + 1;
                conds.push((None, c, s));
                continue;
            };
            let dest = dest.to_string();
            let val = val[2..].parse::<usize>().unwrap();
            let name = s.as_bytes()[0];

            let cond = match s.as_bytes()[1] {
                b'<' => MIN..val,
                b'>' => (val + 1)..MAX + 1,
                _ => unreachable!(),
            };
            conds.push((Some(name), cond, dest));
        }
        res.insert(name.to_string(), conds);
    }

    let mut res2 = Vec::new();
    for rule in rules.lines() {
        let mut vals = Vec::new();
        let s = &rule[1..rule.len() - 1];
        for s in s.split(",") {
            let (name, val) = s.split_once("=").unwrap();
            let name = name.as_bytes()[0];
            let val: usize = val.parse().unwrap();
            vals.push((name, val));
        }
        res2.push(vals);
    }

    Ok((res, res2))
}

#[inline]
fn name_to_idx(name: u8) -> usize {
    match name {
        b'x' => 0,
        b'm' => 1,
        b'a' => 2,
        b's' => 3,
        _ => unreachable!(),
    }
}

fn dp(
    workflows: &HashMap<String, Vec<(Option<u8>, Range<usize>, String)>>,
    wname: String,
    cond_i: usize,
    ranges: Vec<Range<usize>>,
) -> usize {
    let workflow = &workflows[&wname];
    if cond_i >= workflow.len() {
        return 0;
    }
    let cond = &workflow[cond_i];
    let mut res = 0;

    // check false condition
    if let Some(name) = cond.0 {
        let mut false_ranges = ranges.clone();
        let idx = name_to_idx(name);
        let r1 = false_ranges.get_mut(idx).unwrap();
        let r2 = &cond.1;
        let r2 = if r2.start == 1 {
            r2.end..MAX + 1
        } else {
            assert!(r2.end == MAX + 1);
            MIN..r2.start
        };
        let start = r1.start.max(r2.start);
        let end = r1.end.min(r2.end);
        if start >= end {
            unreachable!();
        }
        let new_range = start..end;
        *r1 = new_range;
        res += dp(workflows, wname.clone(), cond_i + 1, false_ranges);
    }

    // check true condition
    let mut true_ranges = ranges;
    if let Some(name) = cond.0 {
        let idx = name_to_idx(name);
        let r1 = true_ranges.get_mut(idx).unwrap();
        let r2 = &cond.1;
        let start = r1.start.max(r2.start);
        let end = r1.end.min(r2.end);
        if start >= end {
            unreachable!();
        }
        let new_range = start..end;
        *r1 = new_range;
    }
    let dst = &cond.2;
    match dst.as_str() {
        "A" => {
            let mut sum: usize = 1;
            for r in true_ranges {
                sum = sum.checked_mul(r.len()).unwrap();
            }
            res += sum;
        }
        "R" => {}
        _ => {
            res += dp(workflows, dst.to_string(), 0, true_ranges);
        }
    }
    res
}

fn part1(input: &str) -> Result<usize> {
    let (workflows, rules) = parse(input)?;
    let mut sum = 0;
    for rule in rules {
        let mut workflow = &workflows[&"in".to_string()];
        'l: loop {
            for cond in workflow {
                let v = if let Some(name) = cond.0 {
                    rule.iter().find(|(n, _)| *n == name).unwrap().1
                } else {
                    1
                };
                if (cond.1).contains(&v) {
                    let dst = &cond.2;
                    match dst.as_str() {
                        "A" => {
                            sum += rule.iter().map(|(_, v)| v).sum::<usize>();
                            break 'l;
                        }
                        "R" => break 'l,
                        n => {
                            workflow = &workflows[n];
                            break;
                        }
                    }
                }
            }
        }
    }
    Ok(sum)
}

fn part2(input: &str) -> Result<usize> {
    let (workflows, _) = parse(input)?;
    let mut ranges = Vec::new();
    for _ in 0..4 {
        ranges.push(MIN..MAX + 1);
    }
    let sum = dp(&workflows, "in".to_string(), 0, ranges);
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
