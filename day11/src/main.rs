use std::{str::FromStr, time::Instant};

use color_eyre::eyre::{Error, Result};
use itertools::Itertools;

#[derive(Debug)]
pub struct Universe {
    pub width: usize,
    pub height: usize,
    pub galaxies: Vec<GridCoord>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct GridCoord {
    pub x: isize,
    pub y: isize,
}

impl FromStr for Universe {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().len();
        let mut galaxies = vec![];
        for (y, l) in s.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c == '#' {
                    galaxies.push(GridCoord {
                        x: x as isize,
                        y: y as isize,
                    });
                }
            }
        }
        Ok(Universe {
            width,
            height,
            galaxies,
        })
    }
}

fn add_expansion(input: &str, universe: &mut Universe, dist: usize) {
    let dist = 1 + dist.checked_sub(2).unwrap_or(0);

    let mut offset = 0;
    for (y, l) in input.lines().enumerate() {
        if !l.contains("#") {
            for galaxy in universe
                .galaxies
                .iter_mut()
                .filter(|g| g.y > y as isize + offset)
            {
                galaxy.y += dist as isize;
            }
            offset += dist as isize;
            universe.height += dist;
        }
    }

    let mut offset = 0;
    for x in 0..universe.width {
        if input.lines().all(|l| l.chars().skip(x).next() == Some('.')) {
            for galaxy in universe
                .galaxies
                .iter_mut()
                .filter(|g| g.x > x as isize + offset)
            {
                galaxy.x += dist as isize;
            }
            universe.width += dist;
            offset += dist as isize;
        }
    }
}

impl GridCoord {
    pub fn manhattan_dist(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut universe: Universe = input.parse()?;
    add_expansion(input, &mut universe, 1);
    let mut sum = 0;
    for (g1, g2) in universe.galaxies.iter().tuple_combinations() {
        let dist = g1.manhattan_dist(g2);
        sum += dist;
    }
    Ok(sum as usize)
}

fn part2(input: &str) -> Result<usize> {
    let mut universe: Universe = input.parse()?;
    add_expansion(input, &mut universe, 1_000_000);
    let mut sum = 0;
    for (g1, g2) in universe.galaxies.iter().tuple_combinations() {
        let dist = g1.manhattan_dist(g2);
        sum += dist;
    }
    Ok(sum as usize)
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
