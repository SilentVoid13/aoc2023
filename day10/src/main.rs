use std::{collections::VecDeque, str::FromStr, time::Instant};

use color_eyre::eyre::{Error, Result};

#[derive(Debug, Clone)]
pub struct Loop {
    pub width: usize,
    pub height: usize,
    pub start: Pipe,
    pub pipes: Vec<Pipe>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridCoord {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone)]
pub struct Pipe {
    pub coord: GridCoord,
    pub neighbours: Vec<GridCoord>,
    pub val: char,
}

fn connecting_neighs(
    width: usize,
    height: usize,
    input: &str,
    c: &GridCoord,
    val: char,
) -> Vec<GridCoord> {
    let left = (GridCoord { x: c.x - 1, y: c.y }, "L-FS");
    let right = (GridCoord { x: c.x + 1, y: c.y }, "J-7S");
    let up = (GridCoord { x: c.x, y: c.y - 1 }, "7|FS");
    let down = (GridCoord { x: c.x, y: c.y + 1 }, "J|LS");

    let dirs = match val {
        '|' => vec![up, down],
        '-' => vec![left, right],
        'L' => vec![up, right],
        'J' => vec![up, left],
        '7' => vec![left, down],
        'F' => vec![down, right],
        // We need points to be counter clockwise, so this order is important
        'S' => vec![down, left, up, right],
        _ => unreachable!(),
    };

    let mut neighs = vec![];
    for dir in dirs {
        let (new_c, possible_vals) = dir;
        if new_c.x < 0 || new_c.x >= width as isize || new_c.y < 0 || new_c.y >= height as isize {
            continue;
        }
        let i = new_c.y as usize * width + new_c.x as usize;
        let new_val = &input[i..i + 1];
        if possible_vals.contains(new_val) {
            neighs.push(new_c);
        }
    }

    neighs
}

impl FromStr for Loop {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().len();
        let map: String = s.lines().collect();
        let mut it = map.chars();
        let mut start = None;

        for y in 0..height {
            for x in 0..width {
                let c = it.next().unwrap();
                if c == 'S' {
                    let coord = GridCoord {
                        x: x as isize,
                        y: y as isize,
                    };
                    let neighs = connecting_neighs(width, height, &map, &coord, c);
                    let p = Pipe {
                        coord,
                        neighbours: neighs,
                        val: c,
                    };
                    start = Some(p);
                }
            }
        }

        let mut pipes: Vec<Pipe> = vec![];
        let mut queue = vec![start.clone().unwrap()];
        let mut visited = vec![];
        while let Some(pipe) = queue.pop() {
            for neigh_c in pipe.neighbours.iter() {
                if pipes.iter().all(|p| p.coord != *neigh_c) && !visited.contains(neigh_c) {
                    visited.push(neigh_c.clone());
                    let i = neigh_c.y as usize * width + neigh_c.x as usize;
                    let val = &map[i..i + 1].chars().next().unwrap();
                    let neighs = connecting_neighs(width, height, &map, &neigh_c, *val);
                    let p = Pipe {
                        coord: neigh_c.clone(),
                        neighbours: neighs,
                        val: *val,
                    };
                    queue.push(p);
                }
            }
            pipes.push(pipe);
        }
        Ok(Loop {
            width,
            height,
            start: start.unwrap(),
            pipes,
        })
    }
}

fn part1(input: &str) -> Result<usize> {
    let lp: Loop = input.parse().unwrap();

    // Basic BFS
    let mut queue = VecDeque::new();
    queue.push_front((&lp.start, 0));
    let mut visited = vec![];
    let mut best_dist = 0;
    while let Some(next) = queue.pop_front() {
        let (pipe, cur_dist) = next;
        if cur_dist > best_dist {
            best_dist = cur_dist;
        }
        for neigh in pipe.neighbours.iter() {
            if !visited.contains(neigh) {
                let p = lp.pipes.iter().find(|pp| pp.coord == *neigh).unwrap();
                queue.push_back((p, cur_dist + 1));
                visited.push(neigh.clone());
            }
        }
    }

    Ok(best_dist)
}

fn part2(input: &str) -> Result<usize> {
    let lp: Loop = input.parse().unwrap();

    // Compute area with the Shoelace formula
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut sum = 0;
    for p in lp.pipes.windows(2) {
        sum += (p[0].coord.y + p[1].coord.y) * (p[0].coord.x - p[1].coord.x)
    }
    let area = 0.5 * sum.abs() as f64;

    // Find interior points with Pick's theorem
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    let i = area - (lp.pipes.len() as f64 / 2.0) + 1.0;

    Ok(i as usize)
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
