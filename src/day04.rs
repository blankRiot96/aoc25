use std::collections::HashSet;

fn get_n_adjacent(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    glen: i32,
    dist: &mut HashSet<(usize, usize)>,
) {
    let irow = row as i32;
    let icol = col as i32;

    let mut adj = 0;
    for off_x in [-1, 0, 1] {
        for off_y in [-1, 0, 1] {
            if off_x == 0 && off_y == 0 {
                continue;
            }

            let trow = irow + off_x;
            let tcol = icol + off_y;
            if (trow < 0) || (trow >= glen) {
                continue;
            }

            if (tcol < 0) || (tcol >= glen) {
                continue;
            }

            let c = grid[trow as usize][tcol as usize];
            if c == '@' {
                adj += 1;
            }
        }
    }

    if adj < 4 {
        dist.insert((row, col));
    }
}

pub fn part_1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let chunks_per_row = (cols + 63) / 64;

    let mut grid: Vec<Vec<u64>> = Vec::with_capacity(rows);
    for line in lines.iter() {
        let mut chunks = vec![0u64; chunks_per_row];
        for (col, &b) in line.as_bytes().iter().enumerate() {
            if b == b'@' {
                let chunk = col / 64;
                let bit = col % 64;
                chunks[chunk] |= 1u64 << bit;
            }
        }
        grid.push(chunks);
    }

    let mut accessible = 0usize;

    let get_bit = |r: isize,
                   col: isize,
                   grid: &Vec<Vec<u64>>,
                   rows: usize,
                   cols: usize,
                   chunks_per_row: usize|
     -> u8 {
        if r < 0 || r as usize >= rows {
            return 0;
        }
        if col < 0 || col as usize >= cols {
            return 0;
        }
        let ci = (col as usize) / 64;
        let bi = (col as usize) % 64;
        ((grid[r as usize][ci] >> bi) & 1) as u8
    };

    for r in 0..rows {
        for chunk in 0..chunks_per_row {
            let mut mask = grid[r][chunk];
            while mask != 0 {
                let bit_idx = mask.trailing_zeros() as usize;
                mask &= mask - 1;

                let col = chunk * 64 + bit_idx;
                if col >= cols {
                    continue;
                }

                let mut count = 0u8;
                let row_isize = r as isize;
                let col_isize = col as isize;

                count += get_bit(
                    row_isize - 1,
                    col_isize - 1,
                    &grid,
                    rows,
                    cols,
                    chunks_per_row,
                );
                count += get_bit(row_isize - 1, col_isize, &grid, rows, cols, chunks_per_row);
                count += get_bit(
                    row_isize - 1,
                    col_isize + 1,
                    &grid,
                    rows,
                    cols,
                    chunks_per_row,
                );

                count += get_bit(row_isize, col_isize - 1, &grid, rows, cols, chunks_per_row);
                count += get_bit(row_isize, col_isize + 1, &grid, rows, cols, chunks_per_row);

                count += get_bit(
                    row_isize + 1,
                    col_isize - 1,
                    &grid,
                    rows,
                    cols,
                    chunks_per_row,
                );
                count += get_bit(row_isize + 1, col_isize, &grid, rows, cols, chunks_per_row);
                count += get_bit(
                    row_isize + 1,
                    col_isize + 1,
                    &grid,
                    rows,
                    cols,
                    chunks_per_row,
                );

                if count < 4 {
                    accessible += 1;
                }
            }
        }
    }

    accessible
}

pub fn part_2(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let glen = grid.len() as i32;

    let mut total = 0usize;
    let mut prev_total = 0usize;

    loop {
        let mut dist: HashSet<(usize, usize)> = HashSet::with_capacity(100);
        for (row, line) in grid.iter().enumerate() {
            for (col, paper) in line.iter().enumerate() {
                if *paper != '@' {
                    continue;
                }
                get_n_adjacent(&grid, row, col, glen, &mut dist);
            }
        }

        let mut count = 0;
        for (row, col) in &dist {
            grid[*row][*col] = '.';
            count += 1;
        }

        // println!("Removed {count} rolls of paper:");
        // for (row, line) in grid.iter().enumerate() {
        //     for (col, paper) in line.iter().enumerate() {
        //         print!("{paper}");
        //     }
        //     println!();
        // }

        if count == 0 {
            break;
        }

        total += count;

        if total == prev_total {
            break;
        }
        prev_total = total;
    }

    total
}
