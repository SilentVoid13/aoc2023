use aoc_utils::{
    grid::Grid,
    point::{Point, ORTHO},
};
use color_eyre::eyre::Result;
use gxhash::{HashMap, HashMapExt, HashSet, HashSetExt};
use std::{
    collections::VecDeque,
    time::Instant,
};

struct GridWrap {
    start: Point,
    end: Point,
    edges: HashMap<Point, HashSet<(Point, usize)>>,
}

fn prune_graph(grid: Grid<u8>, p1: bool) -> GridWrap {
    let start = Point::new(1, 0);
    let end = Point::new((grid.width - 2) as i32, (grid.height - 1) as i32);
    let mut edges: HashMap<Point, HashSet<(Point, usize)>> = HashMap::new();
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    let mut path = HashSet::new();
    path.insert(start);
    q.push_front((start, start, path));

    while let Some((p, mut parent, mut path)) = q.pop_front() {
        let mut neighs = Vec::new();
        for dir in ORTHO {
            let np = p + dir;
            if path.contains(&np) {
                continue;
            }
            if grid.contains(np) {
                if grid[np] == b'#' {
                    continue;
                }
                if p1 && grid[np] != b'.' {
                    let cp = np - p;
                    let valid = match grid[np] {
                        b'^' => cp == Point::new(0, -1),
                        b'<' => cp == Point::new(-1, 0),
                        b'>' => cp == Point::new(1, 0),
                        b'v' => cp == Point::new(0, 1),
                        _ => unreachable!(),
                    };
                    if !valid {
                        continue;
                    }
                }
                neighs.push(np);
            }
        }
        if p == end {
            edges.entry(parent).or_default().insert((p, path.len() - 1));
        }
        if neighs.len() >= 2 {
            edges.entry(parent).or_default().insert((p, path.len() - 1));
            parent = p;
            path.clear();
            path.insert(p);
        }
        for neigh in neighs {
            let mut path = path.clone();
            path.insert(neigh);
            if seen.insert((neigh, parent)) {
                q.push_front((neigh, parent, path));
            }
        }
    }
    GridWrap { start, end, edges }
}

fn parse(input: &str, p1: bool) -> GridWrap {
    let grid = Grid::parse(input);
    prune_graph(grid, p1)
}

fn run(wrap: &GridWrap) -> usize {
    let mut q = VecDeque::new();
    let mut path = HashSet::new();
    let mut best = 0;
    path.insert(wrap.start);
    q.push_back((wrap.start, path, 0));
    while let Some((p, path, cur_cost)) = q.pop_front() {
        if p == wrap.end {
            best = best.max(cur_cost);
            continue;
        }
        for (np, cost) in wrap.edges[&p].iter() {
            if path.contains(np) {
                continue;
            }
            let mut path = path.clone();
            path.insert(*np);
            q.push_back((*np, path, cur_cost + cost));
        }
    }
    best
}

fn part1(input: &str) -> Result<usize> {
    let wrap = parse(input, true);
    Ok(run(&wrap))
}

fn part2(input: &str) -> Result<usize> {
    let wrap = parse(input, false);
    Ok(run(&wrap))
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
