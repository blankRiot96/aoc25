use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;

fn get_adjacent(grid: &[u8], pos: usize, glen: i32, out: &mut HashSet<(i32, i32)>) {
    let ipos = pos as i32;
    let row = ipos / glen;
    let col = ipos % glen;

    let mut adj = 0;

    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }

            let r = row + dx;
            let c = col + dy;

            if r < 0 || r >= glen {
                continue;
            }
            if c < 0 || c >= glen {
                continue;
            }

            let tpos = (r * glen + c) as usize;
            if grid[tpos] == b'@' {
                adj += 1;
            }
        }
    }

    if adj < 4 {
        out.insert((row, col));
    }
}

pub fn part_1(grid_raw: &[u8]) -> usize {
    let mut glen = 0;
    for &c in grid_raw {
        if c == b'\n' {
            break;
        }
        glen += 1;
    }
    let glen_i = glen as i32;

    let mut grid = Vec::with_capacity(glen * glen);
    for &c in grid_raw {
        if c != b'\n' {
            grid.push(c);
        }
    }

    let grid = Arc::new(grid);
    let threads = 8;

    let dist = Arc::new(Mutex::new(HashSet::<(i32, i32)>::new()));

    let rows_per = (glen + threads - 1) / threads;

    let mut handles = Vec::new();

    for t in 0..threads {
        let start_row = t * rows_per;
        if start_row >= glen {
            break;
        }

        let end_row = ((t + 1) * rows_per).min(glen);

        let grid = Arc::clone(&grid);
        let dist = Arc::clone(&dist);

        handles.push(thread::spawn(move || {
            let mut local = HashSet::new();

            for r in start_row..end_row {
                for c in 0..glen {
                    let pos = r * glen + c;
                    if grid[pos] == b'@' {
                        get_adjacent(&grid, pos, glen_i, &mut local);
                    }
                }
            }

            dist.lock().unwrap().extend(local);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    dist.lock().unwrap().len()
}
