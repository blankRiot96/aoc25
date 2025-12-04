use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;

const GRID_CAP: usize = 1500;

fn get_n_adjacent(grid: &[u8], pos: usize, glen: i32, dist: &mut HashSet<(i32, i32)>) {
    let ipos = pos as i32;
    let irow = ipos / glen;
    let icol = ipos % glen;
    let iglen = glen as i32;
    
    let mut adj = 0;
    for off_x in [-1, 0, 1] {
        for off_y in [-1, 0, 1] {
            if off_x == 0 && off_y == 0 {
                continue;
            }

            let trow = irow + off_x;
            let tcol = icol + off_y;
            if (trow < 0) || (trow >= iglen - 1) {
                continue;
            } 

            if (tcol < 0) || (tcol >= iglen) {
                continue;
            }
            
            let tpos = ((iglen * trow) + tcol) as usize;
            if grid[tpos] == b'@' {
                adj += 1; 
            }
        }
    }
    
    if adj < 4 {
        dist.insert((irow, icol));
    }
}

// pub fn part_1(grid: &[u8]) -> usize {
//     let mut glen = 0;
//     for c in grid {
//         glen += 1;
//         if *c == b'\n' {
//             break;
//         }
//     }
//     let mut dist: HashSet<(i32, i32)> = HashSet::with_capacity(1500);
//
//     for (pos, c) in grid.iter().enumerate() {
//         if *c == b'@' {
//             get_n_adjacent(&grid, pos, glen, &mut dist);
//         }
//     }
//
//     dist.len()
// }

pub fn part_1(grid: &[u8]) -> usize {
    let mut glen = 0;
    for &c in grid {
        if c == b'\n' { break; }
        glen += 1;
    }

    let threads = 8;
    let chunk = grid.len() / threads;

    let dist = Arc::new(Mutex::new(HashSet::<(i32, i32)>::with_capacity(1500)));
    let grid = Arc::new(grid.to_vec());

    let mut handles = Vec::new();

    for t in 0..threads {
        let start = t * chunk;
        let end = if t == threads-1 { grid.len() } else { (t+1)*chunk };

        let grid = Arc::clone(&grid);
        let dist = Arc::clone(&dist);

        handles.push(thread::spawn(move || {
            let mut local = HashSet::new();

            for pos in start..end {
                if grid[pos] == b'@' {
                    get_n_adjacent(&grid, pos, glen, &mut local);
                }
            }

            // merge local set into global
            dist.lock().unwrap().extend(local);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    dist.lock().unwrap().len()
}
