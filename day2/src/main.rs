use std::{str::FromStr, time::Instant};

use color_eyre::eyre::{eyre, Error, Result, Context};

#[derive(Debug)]
pub struct Game(Vec<Cube>);

#[derive(Debug)]
pub enum Cube {
    Red(usize),
    Blue(usize),
    Green(usize),
}

#[derive(Debug)]
pub struct Bag {
    pub blue: usize,
    pub red: usize,
    pub green: usize,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s.split_once(": ").unwrap();
        let mut cubes = vec![];
        for split in s.split("; ") {
            for v in split.split(", ") {
                let (num, color) = v.split_once(" ").unwrap();
                let num: usize = num.parse().wrap_err("invalid number")?;
                let cube = match color {
                    "blue" => Cube::Blue(num),
                    "red" => Cube::Red(num),
                    "green" => Cube::Green(num),
                    _ => return Err(eyre!("invalid color"))
                };
                cubes.push(cube);
            }
        }
        Ok(Game(cubes))
    }
}

fn part1(input: &str) -> Result<usize> {
    let games: Vec<Game> = input.lines().map(|l| l.parse().unwrap()).collect();
    let reds = 12;
    let greens = 13;
    let blues = 14;
    let mut count = 0;

    for (i, game) in games.iter().enumerate() {
        let mut cond = true;
        for cube in game.0.iter() {
            cond &= match cube {
                Cube::Red(n) => *n <= reds,
                Cube::Blue(n) => *n <= blues,
                Cube::Green(n) => *n <= greens,
            };
        }
        if cond { count += i + 1 }
    }
    Ok(count)
}

fn part2(input: &str) -> Result<usize> {
    let games: Vec<Game> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut sum = 0;

    for game in games.iter() {
        let mut max_red = 1;
        let mut max_blue = 1;
        let mut max_green = 1;
        for cube in game.0.iter() {
            match cube {
                Cube::Red(n) => max_red = max_red.max(*n),
                Cube::Blue(n) => max_blue = max_blue.max(*n),
                Cube::Green(n) => max_green = max_green.max(*n),
            };
        }
        sum += max_red * max_blue * max_green;
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
