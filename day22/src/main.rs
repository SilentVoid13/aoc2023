use std::{
    ops::{Add, Index, IndexMut, Sub},
    time::Instant,
};

use color_eyre::eyre::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Point { x, y, z }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z
            .cmp(&other.z)
            .then(self.y.cmp(&other.y))
            .then(self.x.cmp(&other.x))
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Brick(Vec<Point>);

#[derive(Debug, Clone)]
struct Grid {
    pub max_x: i32,
    pub max_y: i32,
    pub max_z: i32,
    pub bytes: Vec<u8>,
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, index: Point) -> &mut u8 {
        &mut self.bytes
            [(index.z * self.max_y * self.max_x + index.y * self.max_x + index.x) as usize]
    }
}

impl Index<Point> for Grid {
    type Output = u8;

    fn index(&self, index: Point) -> &Self::Output {
        &self.bytes[(index.z * self.max_y * self.max_x + index.y * self.max_x + index.x) as usize]
    }
}

fn parse(input: &str) -> (Vec<Brick>, Grid) {
    let mut bricks = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    for line in input.lines() {
        let s = line.split("~");
        let mut points = Vec::new();
        for val in s {
            let mut v = val.split(",");
            let x = v.next().unwrap().parse().unwrap();
            let y = v.next().unwrap().parse().unwrap();
            let z = v.next().unwrap().parse().unwrap();
            let point = Point::new(x, y, z);
            points.push(point);
        }
        max_x = max_x.max(points.iter().map(|p| p.x).max().unwrap());
        max_y = max_y.max(points.iter().map(|p| p.y).max().unwrap());
        max_z = max_z.max(points.iter().map(|p| p.z).max().unwrap());

        let mut line = Vec::new();
        let mut p = points[0];
        let mut dir = points[1] - points[0];
        if dir.x != 0 {
            dir.x /= dir.x;
        }
        if dir.y != 0 {
            dir.y /= dir.y;
        }
        if dir.z != 0 {
            dir.z /= dir.z;
        }
        while p != points[1] {
            line.push(p);
            p = p + dir;
        }
        line.push(points[1]);
        bricks.push(Brick(line));
    }

    max_x += 1;
    max_y += 1;
    max_z += 1;
    let bytes = vec![0u8; (max_x * max_y * max_z) as usize];
    let mut plane = Grid {
        max_x,
        max_y,
        max_z,
        bytes,
    };
    for brick in &bricks {
        for p in &brick.0 {
            plane[*p] = 1;
        }
    }

    (bricks, plane)
}

fn fall(bricks: &mut [Brick], grid: &mut Grid) -> usize {
    let mut new_fall = 0;
    let fp = Point::new(0, 0, 1);
    for bi in 0..bricks.len() {
        let mut can_fall = true;
        let mut did_fall = false;
        while can_fall {
            let mut new_brick = bricks[bi].clone();
            for pi in 0..bricks[bi].0.len() {
                let np = bricks[bi].0[pi] - fp;
                if !(bricks[bi].0.contains(&np)) && (grid[np] == 1 || np.z == 0) {
                    can_fall = false;
                    break;
                }
                new_brick.0[pi] = np;
            }
            if can_fall {
                did_fall = true;
                for p in &bricks[bi].0 {
                    grid[*p] = 0;
                }
                bricks[bi] = new_brick;
                for p in &bricks[bi].0 {
                    grid[*p] = 1;
                }
            }
        }
        if did_fall {
            new_fall += 1;
        }
    }
    new_fall
}

fn part1(input: &str) -> Result<usize> {
    let (mut bricks, mut grid) = parse(input);
    bricks.sort();
    // all bricks fall
    fall(&mut bricks, &mut grid);

    // check for each brick if it can be destroyed
    let mut total = 0;
    for bi in 0..bricks.len() {
        let mut bricks = bricks.clone();
        let mut grid = grid.clone();
        for p in bricks.remove(bi).0 {
            grid[p] = 0;
        }
        let r = fall(&mut bricks, &mut grid);
        if r == 0 {
            total += 1;
        }
    }
    Ok(total)
}

fn part2(input: &str) -> Result<usize> {
    let (mut bricks, mut grid) = parse(input);
    bricks.sort();
    // all bricks fall
    fall(&mut bricks, &mut grid);

    let mut total = 0;
    for bi in 0..bricks.len() {
        let mut bricks = bricks.clone();
        let mut grid = grid.clone();
        for p in bricks.remove(bi).0 {
            grid[p] = 0;
        }
        let r = fall(&mut bricks, &mut grid);
        total += r;
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
