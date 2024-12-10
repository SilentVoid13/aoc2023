use std::time::Instant;

use gxhash::{HashSet, HashSetExt};

use aoc_utils::{
    grid::Grid,
    point::{Point, ORTHO},
};
use color_eyre::eyre::Result;

fn parse(input: &str) -> (Grid<u8>, Point) {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();
    (grid, start)
}

fn simulate_steps(grid: &Grid<u8>, start: Point, steps: usize) -> usize {
    let mut queue = HashSet::new();
    queue.insert(start);
    for _ in 0..steps {
        let mut new_queue = HashSet::new();
        for pos in queue {
            for dir in ORTHO {
                let np = pos + dir;
                if grid.contains(np) && (grid[np] == b'.' || grid[np] == b'S') {
                    new_queue.insert(np);
                }
            }
        }
        queue = new_queue;
    }
    queue.len()
}

fn part1(input: &str) -> Result<usize> {
    let (grid, start) = parse(input);
    let r = simulate_steps(&grid, start, 64);
    Ok(r)
}

fn part2(input: &str) -> Result<usize> {
    // this part2 was also not my favorite, as it relies on some manual analysis of the input.
    //
    // we notice that all edges of the grid are only '.' and that we have only '.' on the starting row and
    // starting column.
    // we notice that we start in the middle of the grid.
    // we notice that the number of steps we need to take modulo the grid size is equal to half the
    // grid size
    //
    // this means that we can simulate our BFS manhattan diamond reaching each edge of the grid by
    // slowly computing the inner part, then the corners, then the diagonals.

    let (grid, start) = parse(input);
    let steps = 26501365;
    let mid_steps = grid.width / 2;

    // our assumptions based on manual analysis
    assert!(grid.width == grid.height);
    assert!(start == Point::new(grid.width as i32 / 2, grid.height as i32 / 2));
    assert!(steps % grid.width == mid_steps);

    // number of full grids reached after n steps, not counting the initial grid
    let grid_diam = (steps / grid.width) - 1;

    // number of odd/even grids in the diamond
    let odd_grids = ((grid_diam / 2) * 2 + 1).pow(2);
    let even_grids = ((grid_diam + 1) / 2 * 2).pow(2);

    // 1. we compute the inner part of the diamond
    // we simulate a large amound of odd/even steps to sure make the grid is completely filled

    let odd_points = simulate_steps(&grid, start, grid.width * 2 + 1);
    let even_points = simulate_steps(&grid, start, grid.width * 2);

    // 2. we compute the diamond corners

    let sim_steps = grid.width - 1;
    let t_corner_points = simulate_steps(
        &grid,
        Point::new(start.x, grid.height as i32 - 1),
        sim_steps,
    );
    let b_corner_points = simulate_steps(&grid, Point::new(start.x, 0), sim_steps);
    let r_corner_points = simulate_steps(&grid, Point::new(0, start.y), sim_steps);
    let l_corner_points =
        simulate_steps(&grid, Point::new(grid.width as i32 - 1, start.y), sim_steps);

    // 3. we compute the diamonds diagonals
    // we have two cases: big diagonals and small ones

    let sim_steps = (grid.width / 2) - 1;
    let tr_small_diag = simulate_steps(&grid, Point::new(0, grid.height as i32 - 1), sim_steps);
    let br_small_diag = simulate_steps(&grid, Point::new(0, 0), sim_steps);
    let tl_small_diag = simulate_steps(
        &grid,
        Point::new(grid.width as i32 - 1, grid.height as i32 - 1),
        sim_steps,
    );
    let bl_small_diag = simulate_steps(&grid, Point::new(grid.width as i32 - 1, 0), sim_steps);

    let sim_steps = (grid.width * 3 / 2) - 1;
    let tr_big_diag = simulate_steps(&grid, Point::new(0, grid.height as i32 - 1), sim_steps);
    let br_big_diag = simulate_steps(&grid, Point::new(0, 0), (grid.width * 3 / 2) - 1);
    let tl_big_diag = simulate_steps(
        &grid,
        Point::new(grid.width as i32 - 1, grid.height as i32 - 1),
        sim_steps,
    );
    let bl_big_diag = simulate_steps(&grid, Point::new(grid.width as i32 - 1, 0), sim_steps);

    let r = (odd_grids * odd_points)
        + (even_grids * even_points)
        + b_corner_points
        + t_corner_points
        + r_corner_points
        + l_corner_points
        + (grid_diam + 1) * (tr_small_diag + br_small_diag + tl_small_diag + bl_small_diag)
        + (grid_diam) * (tr_big_diag + br_big_diag + tl_big_diag + bl_big_diag);
    Ok(r)
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
