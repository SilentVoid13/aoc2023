use color_eyre::eyre::Result;
use gxhash::{HashMap, HashMapExt, HashSet};
use rand::seq::SliceRandom;
use std::time::Instant;

type Graph = Vec<Vec<usize>>;

fn parse(input: &str) -> Graph {
    let mut res: HashMap<String, Vec<String>> = Default::default();
    for line in input.lines() {
        let mut s = line.split_whitespace();
        let name = s.next().unwrap();
        let name = name[..name.len() - 1].to_string();

        for n in s {
            let n = n.trim().to_string();
            res.entry(name.clone()).or_default().push(n.clone());
            res.entry(n).or_default().push(name.clone());
        }
    }

    let mut name_map: HashMap<String, usize> = HashMap::new();
    for (i, (name, _)) in res.iter().enumerate() {
        name_map.insert(name.clone(), i);
    }

    let mut res2 = vec![vec![]; res.len()];
    for (name, neighs) in res {
        let i = name_map[&name];
        for neigh in neighs {
            res2[i].push(name_map[&neigh]);
        }
    }
    res2
}

fn karger_min_cut(graph: &Graph) -> (usize, Vec<Vec<usize>>) {
    let mut rng = rand::thread_rng();
    let mut graph = graph.clone();
    let mut vertices: Vec<usize> = (0..graph.len()).collect();
    let mut partitions: HashMap<usize, HashSet<usize>> = vertices
        .iter()
        .map(|&v| (v, vec![v].into_iter().collect()))
        .collect();

    while vertices.len() > 2 {
        // pick a random edge (u, v)
        let u = vertices.choose(&mut rng).copied().unwrap();
        let v = graph[u].choose(&mut rng).copied().unwrap();
        contract_edge(&mut graph, &mut vertices, &mut partitions, u, v);
    }

    let min_cut = graph[vertices[0]].len();
    let partitions = partitions
        .into_values()
        .map(|p| p.into_iter().collect())
        .collect();
    (min_cut, partitions)
}

/// we contract the edge, i.e. we merge u with v
fn contract_edge(
    graph: &mut Graph,
    vertices: &mut Vec<usize>,
    partitions: &mut HashMap<usize, HashSet<usize>>,
    u: usize,
    v: usize,
) {
    let group_v = partitions.remove(&v).unwrap();
    partitions.get_mut(&u).unwrap().extend(group_v);

    // merge neighbours of v into u
    // replace v by u for neighbours
    for ni in 0..graph[v].len() {
        let neighbor = graph[v][ni];
        graph[neighbor].retain(|&node| node != v);
        if neighbor != u {
            graph[neighbor].push(u);
            graph[u].push(neighbor);
        }
    }
    // remove v from vertices
    vertices.retain(|&val| val != v);
    graph[v].clear();
}

fn part1(input: &str) -> Result<usize> {
    let graph = parse(input);
    for i in 0..1000 {
        let (min_cut, partitions) = karger_min_cut(&graph);
        if min_cut == 3 {
            dbg!(i);
            let r = partitions[0].len() * partitions[1].len();
            return Ok(r);
        }
    }
    Ok(0)
}

fn part2(input: &str) -> Result<i64> {
    let _input = parse(input);
    Ok(0)
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
