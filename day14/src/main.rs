use std::time::Instant;

use color_eyre::eyre::Result;

fn tilt_dir(width: usize, height: usize, tilt: &mut [u8], dir_i: usize) {
    const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let direction = DIRECTIONS[dir_i];

    let it: Box<dyn Iterator<Item = usize>> = if dir_i == 0 || dir_i == 1 {
        Box::new(0..tilt.len())
    } else {
        Box::new((0..tilt.len()).rev())
    };

    for ci in it {
        let c = tilt[ci];
        if c == b'O' {
            let mut row_i = (ci % width) as isize;
            let mut col_i = (ci / width) as isize;
            while (col_i + direction.1) >= 0
                && (col_i + direction.1) < height as isize
                && (row_i + direction.0) >= 0
                && (row_i + direction.0) < width as isize
                && tilt[(col_i + direction.1) as usize * width + (row_i + direction.0) as usize]
                    == b'.'
            {
                tilt[col_i as usize * width + row_i as usize] = b'.';
                col_i += direction.1;
                row_i += direction.0;
                tilt[col_i as usize * width + row_i as usize] = b'O';
            }
        }
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut tilt = input.as_bytes().to_vec();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len() + 1;

    tilt_dir(width, height, &mut tilt, 0);
    let mut total = 0;
    for (li, l) in tilt.split(|&c| c == b'\n').enumerate() {
        total += l.iter().filter(|&c| *c == b'O').count() * (height - li);
    }
    Ok(total)
}

fn part2(input: &str) -> Result<usize> {
    let tilt = input.as_bytes().to_vec();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len() + 1;

    let cycle = |t: &mut Vec<u8>| {
        for dir_i in 0..4 {
            tilt_dir(width, height, t, dir_i);
        }
    };

    let mut tortoise = tilt.clone();
    let mut hare = tilt.clone();

    // find a cycle
    loop {
        cycle(&mut tortoise);
        cycle(&mut hare);
        cycle(&mut hare);
        if tortoise == hare {
            break;
        }
    }

    // find mu, the start of the cycle
    let mut mu = 0;
    tortoise = tilt.clone();
    while tortoise != hare {
        cycle(&mut tortoise);
        cycle(&mut hare);
        mu += 1;
    }

    // find the cycle len
    let mut cycle_len = 1;
    let mut hare = tortoise.clone();
    cycle(&mut hare);
    while tortoise != hare {
        cycle(&mut hare);
        cycle_len += 1;
    }

    let remaining = (1000000000 - mu) % cycle_len;
    for _ in 0..remaining {
        cycle(&mut tortoise);
    }

    let mut total = 0;
    for (li, l) in tortoise.split(|&c| c == b'\n').enumerate() {
        total += l.iter().filter(|&c| *c == b'O').count() * (height - li);
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
