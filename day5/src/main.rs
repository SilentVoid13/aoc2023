use std::{str::FromStr, time::Instant};

use color_eyre::eyre::{ContextCompat, Error, Result};

#[derive(Debug)]
struct Map {
    pub src: usize,
    pub dst: usize,
    pub size: usize,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Vec<Map>>,
}

impl FromStr for Almanac {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split("\n\n");
        let (_, seeds) = it
            .next()
            .wrap_err("no seeds")?
            .split_once(": ")
            .wrap_err("no seeds")?;
        let seeds = seeds
            .split(" ")
            .filter_map(|v| v.parse::<usize>().ok())
            .collect();
        let mut maps = vec![];
        for s in it {
            let mut m = vec![];
            for l in s.lines().skip(1) {
                let vals: Vec<usize> = l
                    .split(" ")
                    .filter_map(|v| v.parse::<usize>().ok())
                    .collect();
                let map = Map {
                    src: vals[1],
                    dst: vals[0],
                    size: vals[2],
                };
                m.push(map);
            }
            maps.push(m);
        }
        Ok(Almanac { seeds, maps })
    }
}

fn find_location(seed: usize, maps: &Vec<Vec<Map>>) -> usize {
    let mut location = seed;
    for map in maps.iter() {
        for m in map.iter() {
            let r = m.src..m.src + m.size;
            if r.contains(&location) {
                location = m.dst + (location - m.src);
                break;
            }
        }
    }
    location
}

fn part1(input: &str) -> Result<usize> {
    let almanac: Almanac = input.parse()?;
    let mut locations = vec![];
    for seed in almanac.seeds.iter() {
        let location = find_location(*seed, &almanac.maps);
        locations.push(location);
    }
    let min = locations.iter().min().wrap_err("no min")?;
    Ok(*min)
}

fn part2(input: &str) -> Result<usize> {
    let almanac: Almanac = input.parse()?;
    let mut locations = vec![];
    for chunk in almanac.seeds.chunks(2) {
        for seed in chunk[0]..chunk[0] + chunk[1] {
            let location = find_location(seed, &almanac.maps);
            locations.push(location);
        }
    }
    let min = locations.iter().min().wrap_err("no min")?;
    Ok(*min)
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
