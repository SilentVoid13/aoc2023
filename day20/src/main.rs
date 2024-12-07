use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

use color_eyre::eyre::Result;

#[derive(Debug, Clone)]
struct Module {
    name: String,
    typ: ModuleType,
    cast: Vec<usize>,
}

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcast,
    Flip,
    Inv(Vec<usize>),
    Empty,
}

fn parse(input: &str) -> (usize, Vec<Module>) {
    let mut res = Vec::new();
    let mut name_map = HashMap::new();
    let mut args = Vec::new();
    let mut bi = None;

    for (i, l) in input.lines().enumerate() {
        let (name, a) = l.split_once(" -> ").unwrap();
        let a: Vec<&str> = a.split(", ").collect();
        args.push(a.clone());
        if name == "broadcaster" {
            bi = Some(i);
            let m = Module {
                name: name.to_string(),
                typ: ModuleType::Broadcast,
                cast: Default::default(),
            };
            res.push(m);
        } else if let Some(n) = name.strip_prefix("%") {
            let m = Module {
                name: n.to_string(),
                typ: ModuleType::Flip,
                cast: Default::default(),
            };
            name_map.insert(n, i);
            res.push(m);
        } else {
            let n = &name[1..];
            let m = Module {
                name: n.to_string(),
                typ: ModuleType::Inv(vec![]),
                cast: Default::default(),
            };
            name_map.insert(n, i);
            res.push(m);
        }
    }
    for i in 0..res.len() {
        for arg in &args[i] {
            let v = if let Some(v) = name_map.get(arg) {
                *v
            } else {
                let i = res.len();
                res.push(Module {
                    name: arg.to_string(),
                    typ: ModuleType::Empty,
                    cast: vec![],
                });
                i
            };
            if let ModuleType::Inv(ref mut inputs) = res[v].typ {
                inputs.push(i);
            }
            res[i].cast.push(v);
        }
    }

    (bi.unwrap(), res)
}

fn step(
    bi: usize,
    modules: &[Module],
    flip_state: &mut [bool],
    inv_state: &mut [HashMap<usize, bool>],
    lows: &mut usize,
    highs: &mut usize,
) {
    let mut queue = VecDeque::new();
    // we press the button
    *lows += 1;
    queue.push_back((bi, false));
    while let Some((mi, mut high)) = queue.pop_front() {
        let module = &modules[mi];
        match module.typ {
            ModuleType::Broadcast => {}
            ModuleType::Flip => {
                // high pulse, it is ignored and nothing happens
                if high {
                    continue;
                }
                let flipped = flip_state[mi];
                high = !flipped;
                flip_state[mi] = !flipped;
            }
            ModuleType::Inv(ref inputs) => {
                let last_pulses = &mut inv_state[mi];
                // high pulses for all inputs, send low pulse
                high = !inputs.iter().all(|i| *last_pulses.get(i).unwrap_or(&false));
            }
            ModuleType::Empty => {}
        }

        for mii in &module.cast {
            if high {
                *highs += 1;
            } else {
                *lows += 1;
            }

            if let ModuleType::Inv(_) = modules[*mii].typ {
                inv_state[*mii].insert(mi, high);
            }
            queue.push_back((*mii, high));
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn lcm_multi(numbers: &[usize]) -> usize {
    numbers.iter().copied().reduce(lcm).unwrap_or(1)
}

fn part1(input: &str) -> Result<usize> {
    let (bi, modules) = parse(input);
    let mut lows = 0;
    let mut highs = 0;
    let mut flip_state = vec![false; modules.len()];
    let mut inv_state = vec![HashMap::new(); modules.len()];

    for _ in 0..1000 {
        step(
            bi,
            &modules,
            &mut flip_state,
            &mut inv_state,
            &mut lows,
            &mut highs,
        );
    }
    Ok(highs * lows)
}

fn part2(input: &str) -> Result<usize> {
    // not a huge fan of part 2, we need to manually analyze the input data to speed up the process
    //
    // if we take a look at the graph, we have 4 binary counters that run in parallel
    // once the number is reached for a counter, a high pulse is sent and the counter is reset
    //
    // the 'rx' target receives a low pulse when all 4 binary counters are synchronized, i.e.
    // when they all receive a high pulse at the same time
    //
    // this basically means that we need to first find out the binary number for the 4 counters and
    // then compute the least common multiple of the 4 numbers

    let (bi, modules) = parse(input);
    let mut vals = vec![];
    for mi in &modules[bi].cast {
        let module = &modules[*mi];
        let mut s = String::new();
        let mut node = *mi;
        loop {
            let m = &modules[node];
            if m.cast.len() == 2 || s.len() == 11 {
                s.push('1');
            } else {
                s.push('0');
            }
            let mut found = false;
            for n in &m.cast {
                let m = &modules[*n];
                if let ModuleType::Flip = m.typ {
                    found = true;
                    node = *n;
                }
            }
            if !found {
                break;
            }
        }
        // last bit is always 1
        let s = "1".to_string() + &s.chars().rev().collect::<String>()[1..];
        let v = usize::from_str_radix(&s, 2).unwrap();
        vals.push(v);
    }
    let lcm = lcm_multi(&vals);
    Ok(lcm)
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
