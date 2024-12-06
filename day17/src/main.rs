use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
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
    pub fn dirs(&self) -> [(i32, i32, Direction); 3] {
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
    row: u32,
    col: u32,
    direction: Direction,
    count: u32,
}

type Cache = Vec<Option<Vec<Node>>>;

pub struct Graph<'a> {
    input: &'a [u8],
    width: u32,
    height: u32,
    max_count: u32,
    min_count: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct QueueEntry {
    real_cost: u32,
    heur_cost: u32,
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
    #[inline]
    pub fn idx(&self, row: u32, col: u32) -> u32 {
        col * self.width + row
    }

    #[inline]
    pub fn eidx(&self, node: Node) -> usize {
        ((self.idx(node.row, node.col) * 4 * self.max_count)
            + (node.direction as u32 * 4)
            + node.count) as usize
    }

    pub fn edges(&mut self, node: Node, edge_cache: &mut Cache) -> usize {
        let i = self.eidx(node);
        if edge_cache[i].is_some() {
            return i;
        }

        let mut res = Vec::new();
        for dir in node.direction.dirs().iter() {
            if dir.2 == node.direction && node.count + 1 > self.max_count {
                continue;
            }
            if dir.2 != node.direction && node.count < self.min_count {
                continue;
            }
            let new_row = node.row as i32 + dir.0;
            let new_col = node.col as i32 + dir.1;
            if new_row >= 0
                && new_row < self.width as i32 - 1
                && new_col >= 0
                && new_col < self.height as i32
            {
                let count = if dir.2 == node.direction {
                    node.count + 1
                } else {
                    1
                };
                res.push(Node {
                    row: new_row as u32,
                    col: new_col as u32,
                    direction: dir.2,
                    count,
                });
            }
        }

        edge_cache[i] = Some(res);
        i
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
    graph: &mut Graph,
    starts: &[Node],
    target: Node,
    heuristic: impl Fn(&Graph, Node) -> u32,
) -> Option<(u32, Vec<Node>)> {
    let mut prevs: HashMap<Node, (Option<Node>, i32)> = HashMap::new();
    //let mut queue = BinaryHeap::new();
    let mut queue = (0..100)
        .map(|_| Vec::with_capacity(1000))
        .collect::<Vec<_>>();

    for start in starts {
        /*
        queue.push(QueueEntry {
            real_cost: 0,
            heur_cost: heuristic(graph, *start),
            node: *start,
        });
        */
        queue[0].push(QueueEntry {
            real_cost: 0,
            heur_cost: heuristic(graph, *start),
            node: *start,
        });
        prevs.insert(*start, (None, 0));
    }

    let mut edge_cache = vec![None; (graph.width * graph.height * 4 * graph.max_count) as usize];
    let mut prevs2 = vec![None; (graph.width * graph.height * 4 * graph.max_count) as usize];
    let mut found = None;

    let mut bucket = 0;

    //while let Some(e) = queue.pop() {
    'l: loop {
        while let Some(e) = queue[bucket % 100].pop() {
            if e.node.row == target.row && e.node.col == target.col && e.node.count >= graph.min_count {
                found = Some((e.node, e.real_cost));
                break 'l;
            }
            let ei = graph.edges(e.node, &mut edge_cache);
            for edge in edge_cache[ei].as_ref().unwrap() {
                let c = graph.input[graph.idx(edge.row, edge.col) as usize];
                // little trick to convert ascii to number
                let new_rcost = e.real_cost + (c - 48) as u32;

                if prevs2[graph.eidx(*edge)].map_or(true, |c| new_rcost < c) {
                //if prevs.get(edge).map_or(true, |(_, c)| new_rcost < *c) {
                    //prevs.insert(*edge, (Some(e.node), new_rcost));
                    prevs2[graph.eidx(*edge)] = Some(new_rcost);

                    let new_hcost = new_rcost + heuristic(graph, *edge);

                    queue[new_hcost as usize % 100].push(QueueEntry {
                        real_cost: new_rcost,
                        heur_cost: new_hcost,
                        node: *edge,
                    });
                    /*
                    queue.push(QueueEntry {
                        real_cost: new_rcost,
                        heur_cost: new_hcost,
                        node: *edge,
                    });
                    */
                }
            }
        }
        bucket += 1;
    }

    // reconstruct path
    let (target_id, target_cost) = found?;
    let mut path = vec![target_id];
    /*
    let mut cur = target_id;
    while !starts.contains(&cur) {
        let (prev_node, _) = prevs[&cur];
        cur = prev_node?;
        path.push(cur);
    }
    path.reverse();
    */
    Some((target_cost, path))
}

fn part1(input: &str) -> Result<usize> {
    let width = input.lines().next().unwrap().len() + 1;
    let height = input.lines().count();
    let mut g = Graph {
        input: input.as_bytes(),
        width: width as u32,
        height: height as u32,
        max_count: 3,
        min_count: 0,
    };
    let end = Node {
        row: g.width - 2,
        col: g.height - 1,
        direction: Direction::South,
        count: 0,
    };
    let res = astar(&mut g, &STARTS, end, |_, _| 0);
    let (cost, _) = res.unwrap();

    Ok(cost as usize)
}

fn part2(input: &str) -> Result<usize> {
    let width = input.lines().next().unwrap().len() + 1;
    let height = input.lines().count();
    let mut g = Graph {
        input: input.as_bytes(),
        width: width as u32,
        height: height as u32,
        max_count: 10,
        min_count: 4,
    };
    let end = Node {
        row: g.width - 2,
        col: g.height - 1,
        direction: Direction::South, // not used
        count: 0,
    };
    let res = astar(
        &mut g,
        &STARTS,
        end,
        // manhattan distance
        //|_, node| (width as u32 - 2 - node.row) + (height as u32 - 1 - node.col),
        |_, _| 0,
    );
    let (cost, _) = res.unwrap();
    Ok(cost as usize)
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
