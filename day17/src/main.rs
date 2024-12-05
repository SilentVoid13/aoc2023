use std::{
    collections::{BinaryHeap, HashMap},
    time::Instant,
};

use color_eyre::eyre::Result;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn dirs(&self) -> [(isize, isize, Direction); 3] {
        match self {
            Direction::North => [
                (0, -1, Direction::North),
                (-1, 0, Direction::West),
                (1, 0, Direction::East),
            ],
            Direction::South => [
                (0, 1, Direction::South),
                (-1, 0, Direction::West),
                (1, 0, Direction::East),
            ],
            Direction::East => [
                (1, 0, Direction::East),
                (0, -1, Direction::North),
                (0, 1, Direction::South),
            ],
            Direction::West => [
                (-1, 0, Direction::West),
                (0, -1, Direction::North),
                (0, 1, Direction::South),
            ],
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Node {
    row: usize,
    col: usize,
    direction: Direction,
    count: usize,
}

pub struct Graph<'a> {
    input: &'a [u8],
    width: usize,
    height: usize,
    max_count: usize,
    min_count: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct QueueEntry {
    real_cost: usize,
    heur_cost: usize,
    node: Node,
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // NOTE: we reverse the order to get smaller cost at the top
        other.heur_cost.cmp(&self.heur_cost)
    }
}

impl Graph<'_> {
    pub fn idx(&self, row: usize, col: usize) -> usize {
        col * self.width + row
    }

    pub fn edges(&self, node: Node) -> Vec<Node> {
        let mut res = Vec::new();
        for dir in node.direction.dirs().iter() {
            if dir.2 == node.direction && node.count + 1 > self.max_count {
                continue;
            }
            if dir.2 != node.direction && node.count < self.min_count {
                continue;
            }
            let new_row = node.row as isize + dir.0;
            let new_col = node.col as isize + dir.1;
            if new_row >= 0
                && new_row < self.width as isize - 1
                && new_col >= 0
                && new_col < self.height as isize
            {
                let count = if dir.2 == node.direction {
                    node.count + 1
                } else {
                    1
                };
                res.push(Node {
                    row: new_row as usize,
                    col: new_col as usize,
                    direction: dir.2,
                    count,
                });
            }
        }
        res
    }
}

const STARTS: [Node; 2] = [
    Node {
        row: 0,
        col: 0,
        direction: Direction::East,
        count: 0,
    },
    Node {
        row: 0,
        col: 0,
        direction: Direction::South,
        count: 0,
    },
];

pub fn astar(
    graph: &Graph,
    starts: &[Node],
    target: Node,
    heuristic: impl Fn(&Graph, Node) -> usize,
) -> Option<(usize, Vec<Node>)> {
    let mut prevs = HashMap::new();
    let mut queue = BinaryHeap::new();

    for start in starts {
        queue.push(QueueEntry {
            real_cost: 0,
            heur_cost: heuristic(graph, *start),
            node: *start,
        });
        prevs.insert(*start, (None, 0));
    }

    let mut found = None;
    while let Some(e) = queue.pop() {
        if e.node.row == target.row && e.node.col == target.col && e.node.count >= graph.min_count {
            found = Some((e.node, e.real_cost));
            break;
        }
        for edge in graph.edges(e.node) {
            let c = graph.input[graph.idx(edge.row, edge.col)];
            assert!(c != b'\n');
            // little trick to convert ascii to number
            let new_rcost = e.real_cost + (c - 48) as usize;
            if prevs.get(&edge).map_or(true, |(_, c)| new_rcost < *c) {
                let new_hcost = new_rcost + heuristic(graph, edge);
                prevs.insert(edge, (Some(e.node), new_rcost));
                queue.push(QueueEntry {
                    real_cost: new_rcost,
                    heur_cost: new_hcost,
                    node: edge,
                });
            }
        }
    }

    // reconstruct path
    let (target_id, target_cost) = found?;
    let mut path = vec![target_id];
    let mut cur = target_id;
    while !starts.contains(&cur) {
        let (prev_node, _) = prevs[&cur];
        cur = prev_node?;
        path.push(cur);
    }
    path.reverse();
    Some((target_cost, path))
}

fn part1(input: &str) -> Result<usize> {
    let g = Graph {
        input: input.as_bytes(),
        width: input.lines().next().unwrap().len() + 1,
        height: input.lines().count(),
        max_count: 3,
        min_count: 0,
    };
    let res = astar(
        &g,
        &STARTS,
        Node {
            row: g.width - 2,
            col: g.height - 1,
            direction: Direction::South,
            count: 0,
        },
        |_, _| 0,
    );
    let (cost, _) = res.unwrap();

    Ok(cost)
}

fn part2(input: &str) -> Result<usize> {
    let g = Graph {
        input: input.as_bytes(),
        width: input.lines().next().unwrap().len() + 1,
        height: input.lines().count(),
        max_count: 10,
        min_count: 4,
    };
    let res = astar(
        &g,
        &STARTS,
        Node {
            row: g.width - 2,
            col: g.height - 1,
            direction: Direction::South, // not used
            count: 0,
        },
        |_, _| 0,
    );
    let (cost, _) = res.unwrap();
    Ok(cost)
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
