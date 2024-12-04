use std::{collections::HashSet, time::Instant};

use color_eyre::eyre::Result;

fn solve_with_start(input: &[u8], width: isize, height: isize, start: (isize, usize)) -> usize {
    const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut queue: Vec<(isize, usize)> = Vec::new();
    queue.push(start);
    let mut energized = HashSet::new();
    let mut visited = HashSet::new();

    let mut update_pos = |queue: &mut Vec<(isize, usize)>,
                          row: &mut isize,
                          col: &mut isize,
                          dir_i: usize,
                          push: bool| {
        if *row >= 0 || *row < width || *col >= 0 || *col < height {
            energized.insert(*col * width + *row);
        }
        let dir = DIRECTIONS[dir_i];
        *row += dir.0;
        *col += dir.1;

        let c = *col * width + *row;
        if push && !visited.contains(&(c, dir_i)) {
            visited.insert((c, dir_i));
            queue.push((c, dir_i));
        }
    };

    while let Some(e) = queue.pop() {
        let (ci, dir_i) = e;
        let mut row = ci % width;
        let mut col = ci / width;

        while input.get((col * width + row) as usize) == Some(&b'.') {
            update_pos(&mut queue, &mut row, &mut col, dir_i, false);
        }
        match input.get((col * width + row) as usize) {
            Some(b'\\') => {
                let new_dir = match dir_i {
                    0 => 2,
                    1 => 3,
                    2 => 0,
                    3 => 1,
                    _ => unreachable!(),
                };
                update_pos(&mut queue, &mut row, &mut col, new_dir, true);
            }
            Some(b'/') => {
                let new_dir = match dir_i {
                    0 => 3,
                    1 => 2,
                    2 => 1,
                    3 => 0,
                    _ => unreachable!(),
                };
                update_pos(&mut queue, &mut row, &mut col, new_dir, true);
            }
            Some(b'-') => {
                if dir_i >= 2 {
                    let mut r1 = row;
                    let mut c1 = col;
                    update_pos(&mut queue, &mut r1, &mut c1, 0, true);
                    let mut r1 = row;
                    let mut c1 = col;
                    update_pos(&mut queue, &mut r1, &mut c1, 1, true);
                } else {
                    update_pos(&mut queue, &mut row, &mut col, dir_i, true);
                }
            }
            Some(b'|') => {
                if dir_i <= 1 {
                    let mut r1 = row;
                    let mut c1 = col;
                    update_pos(&mut queue, &mut r1, &mut c1, 2, true);

                    let mut r1 = row;
                    let mut c1 = col;
                    update_pos(&mut queue, &mut r1, &mut c1, 3, true);
                } else {
                    update_pos(&mut queue, &mut row, &mut col, dir_i, true);
                }
            }
            None => {}
            Some(b'\n') => {}
            Some(_) => unreachable!(),
        }
    }
    energized.len()
}

fn part1(input: &str) -> Result<usize> {
    let height = input.lines().count() as isize;
    let width = input.lines().next().unwrap().len() as isize + 1;
    let s = solve_with_start(input.as_bytes(), width, height, (0, 0));
    Ok(s)
}

fn part2(input: &str) -> Result<usize> {
    let height = input.lines().count() as isize;
    let width = input.lines().next().unwrap().len() as isize + 1;

    let mut best = 0;
    for i in 0..width {
        let s = solve_with_start(input.as_bytes(), width, height, (i, 2));
        best = best.max(s);
        let s = solve_with_start(input.as_bytes(), width, height, (height * width - i, 3));
        best = best.max(s);
    }
    for i in 0..height {
        let s = solve_with_start(input.as_bytes(), width, height, (width * i, 0));
        best = best.max(s);
        let s = solve_with_start(input.as_bytes(), width, height, (width * (i + 1) - 2, 1));
        best = best.max(s);
    }
    Ok(best)
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
