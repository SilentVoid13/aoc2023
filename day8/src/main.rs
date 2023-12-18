use std::{collections::HashMap, time::Instant};

use color_eyre::eyre::Result;

enum Direction {
    Left,
    Right,
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<&str, [String; 2]>) {
    let mut it = input.lines();
    let dirs: Vec<Direction> = it
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect();
    it.next();

    let mut graph = HashMap::new();
    for l in it {
        let (node, rest) = l.split_once(" = ").unwrap();
        let (n1, n2) = rest.split_once(", ").unwrap();
        let n1 = n1[1..].to_string();
        let mut n2_c = n2.chars();
        n2_c.next_back();
        let n2 = n2_c.as_str().to_string();
        graph.insert(node, [n1, n2]);
    }

    (dirs, graph)
}

fn part1(input: &str) -> Result<usize> {
    let (dirs, graph) = parse_input(input);
    let mut steps = 0;
    let mut it = dirs.iter().cycle();
    let mut cur = "AAA";
    while cur != "ZZZ" {
        let neigh = &graph[cur];
        let dir = it.next().unwrap();
        cur = match dir {
            Direction::Left => &neigh[0],
            Direction::Right => &neigh[1],
        };
        steps += 1;
    }
    Ok(steps)
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn part2(input: &str) -> Result<usize> {
    let (dirs, graph) = parse_input(input);
    let mut lengths = vec![];

    // Compute loop length for each starting node
    for node in graph.keys() {
        if node.ends_with("A") {
            let mut it = dirs.iter().cycle();
            let mut l = 0;
            let mut start = false;
            let mut cur = node.to_string();
            loop {
                let neigh = &graph[cur.as_str()];
                let dir = it.next().unwrap();
                cur = match dir {
                    Direction::Left => neigh[0].to_string(),
                    Direction::Right => neigh[1].to_string(),
                };
                if start {
                    l += 1;
                }
                if cur.ends_with("Z") {
                    if start {
                        break;
                    }
                    start = true;
                }
            }
            lengths.push(l);
        }
    }

    // compute the lcm between all loop lengths to find the first intersection
    let lcm = lcm(&lengths);
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
