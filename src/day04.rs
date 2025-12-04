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
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let glen = grid.len() as i32;

    let mut dist = HashSet::with_capacity(1500);

    for (row, line) in grid.iter().enumerate() {
        for (col, paper) in line.iter().enumerate() {
            if *paper != '@' {
                continue;
            }
            get_n_adjacent(&grid, row, col, glen, &mut dist);
        }
    }

    dist.len()
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
