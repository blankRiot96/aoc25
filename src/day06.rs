
use std::simd::u64x64;
use std::simd::num::SimdUint;
use std::time::Instant;
use std::str;

pub fn part_1(input: &str) -> u64 {
    let ops_line = input.lines().last().unwrap().as_bytes();
    let mut ops: Vec<u8> = Vec::with_capacity(1000);
    for &b in ops_line {
        if b != b' ' {
            ops.push(b);
        }
    }
    let ops_ptr = ops.as_ptr();
    let rows: Vec<&str> = input.lines().take(4).collect();
    let w = ops.len();
    let n_vecs = (w + 63) / 64;

    let mut add_rows: Vec<Vec<u64x64>> = vec![vec![u64x64::splat(0); n_vecs]; 4];
    let mut mult_rows: Vec<Vec<u64x64>> = vec![vec![u64x64::splat(0); n_vecs]; 4];

    for row_index in 0..4 {
        let mut add_pointer = 0usize;
        let mut mult_pointer = 0usize;
        let bytes = rows[row_index].as_bytes();
        let mut i = 0usize;
        let mut col = 0usize;

        while i < bytes.len() {
            while i < bytes.len() && bytes[i] == b' ' {
                i += 1;
            }
            if i >= bytes.len() { break; }

            let mut x = 0u64;
            while i < bytes.len() {
                let b = bytes[i];
                if b < b'0' || b > b'9' { break; }
                x = x * 10 + (b - b'0') as u64;
                i += 1;
            }

            let op = unsafe { *ops_ptr.add(col) };
            col += 1;

            if op == b'+' {
                let v = add_pointer >> 6;
                let p = add_pointer & 63;
                unsafe {
                    add_rows
                        .get_unchecked_mut(row_index)
                        .get_unchecked_mut(v)
                        .as_mut_array()[p] = x;
                }
                add_pointer += 1;
            } else {
                let v = mult_pointer >> 6;
                let p = mult_pointer & 63;
                unsafe {
                    mult_rows
                        .get_unchecked_mut(row_index)
                        .get_unchecked_mut(v)
                        .as_mut_array()[p] = x;
                }
                mult_pointer += 1;
            }

            while i < bytes.len() && bytes[i] == b' ' {
                i += 1;
            }
        }
    }

    let mut total = 0;

    for vi in 0..n_vecs {
        let res_add = add_rows[0][vi] + add_rows[1][vi] + add_rows[2][vi] + add_rows[3][vi];
        total += res_add.reduce_sum();
        let res_mult = mult_rows[0][vi] * mult_rows[1][vi] * mult_rows[2][vi] * mult_rows[3][vi];
        total += res_mult.reduce_sum();
    }

    total
}




fn transpose(grid: Vec<Vec<&u8>>) -> Vec<Vec<u8>> {
    let grid_n_rows = grid.len();
    let grid_n_cols = grid[0].len();
    let mut posed = vec![vec![0u8; grid_n_rows]; grid_n_cols];
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            posed[grid_n_cols - col_index - 1][row_index] = **col;
        }
    }
    posed
}

#[inline(always)]
fn fastu64(s: Vec<u8>) -> u64 {
    let mut x = 0u64;
    for b in s {
        if b == b' ' || b == 0 {
            continue;
        }
        x = x * 10 + (b - b'0') as u64;
    }
    x
}

pub fn part_2(input: &str) -> u64 {
    let start = Instant::now();
    let mut lines = input.lines();
    let grid: Vec<Vec<&u8>> = vec![
        lines.next().unwrap().as_bytes().iter().collect::<Vec<&u8>>(),
        lines.next().unwrap().as_bytes().iter().collect::<Vec<&u8>>(),
        lines.next().unwrap().as_bytes().iter().collect::<Vec<&u8>>(),
        lines.next().unwrap().as_bytes().iter().collect::<Vec<&u8>>(),
        // lines.next().unwrap().split_ascii_whitespace().map(|c| &c.as_bytes()[0]).collect::<Vec<&u8>>(),

        lines.next().unwrap().as_bytes().iter().collect::<Vec<&u8>>(),
    ];
    // let n_vecs = (grid[4].len() + 63) / 64;
    let mut rows = transpose(grid);
    
    // let mut add_rows: Vec<Vec<u64x64>> = vec![vec![u64x64::splat(0); n_vecs]; 4];
    // let mut mult_rows: Vec<Vec<u64x64>> = vec![vec![u64x64::splat(1); n_vecs]; 4];
    //
    // let mut add_pointer = 0usize;
    // let mut mult_pointer = 0usize;
    
    let mut sum = 0;
    let mut nums: Vec<u64> = Vec::new();
    for num in rows {
        // println!("Line: {}", str::from_utf8(&num).unwrap());
        let op = *num.last().unwrap();
        if op == b'+' {
            let n = fastu64(num[..num.len() - 1].to_vec());
            if n != 0 {
                nums.push(n);
            }

            // println!("Operation with: {}", op as char);
            // println!("Numbers consumed: {nums:?}");
            sum += nums.iter().sum::<u64>();
            // for (row_index, nu) in nums.iter().enumerate() {
            //     add_rows[row_index][add_pointer / 64][add_pointer % 64] = *nu;
            // }
            // add_pointer += 1;
            nums.clear();
        } else if op == b'*' {
            let n = fastu64(num[..num.len() - 1].to_vec());
            if n != 0 {
                nums.push(n);
            }

            // println!("Operation with: {}", op as char);
            // println!("Numbers consumed: {nums:?}");
            sum += nums.iter().product::<u64>();
            // for (row_index, nu) in nums.iter().enumerate() {
            //     mult_rows[row_index][mult_pointer / 64][mult_pointer % 64] = *nu;
            // }
            // mult_pointer += 1;
            nums.clear();
        } else {
            let n = fastu64(num[..num.len() - 1].to_vec());
            if n != 0 {
                nums.push(n);
            }
        }
    }

    println!("Total: {}µs", start.elapsed().as_micros());
    
    // let start = Instant::now();
    // let mut total = 0;
    //
    // for vi in 0..n_vecs {
    //     let res_add = (add_rows[0][vi] + add_rows[1][vi] + add_rows[2][vi] + add_rows[3][vi]).reduce_sum();
    //     if res_add == 0 {
    //         break;
    //     }
    //     total += res_add;
    // } 
    // for vi in 0..n_vecs {
    //     let res_mult = (mult_rows[0][vi] * mult_rows[1][vi] * mult_rows[2][vi] * mult_rows[3][vi]).reduce_sum();
    //     if res_mult == 1 {
    //         println!("{:?}", mult_rows[0][vi]);
    //         break;
    //     }
    //     total += res_mult;
    // }
    //
    // println!("Calc: {}µs", start.elapsed().as_micros());
    
    // println!("Expected: 9170286552289");
    // println!("Got:      {sum}");
    // println!("Got:      {total}");
    sum
}

