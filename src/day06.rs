
use std::simd::u64x64;
use std::simd::num::SimdUint;

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


pub fn part_2(input: &str) -> u64 {
    let ops_line = input.lines().last().unwrap().as_bytes();
    let mut ops: Vec<u8> = Vec::with_capacity(1000);
    let mut reachers: Vec<usize> = Vec::with_capacity(1000);
    let mut reach_counter = 0usize;

    for &b in ops_line {
        if b != b' ' {
            ops.push(b);
            reachers.push(reach_counter);
        }
        reach_counter += 1;
    }

    let ops_ptr = ops.as_ptr();
    let rows: Vec<&str> = input.lines().take(4).collect();
    let w = ops.len();
    let n_vecs = (w + 63) / 64;

    let mut add_rows: Vec<Vec<u64x64>> = vec![vec![u64x64::splat(0); n_vecs]; 4];
    let mut mult_rows: Vec<Vec<u64x64>> = vec![vec![u64x64::splat(0); n_vecs]; 4];
    
    let mut num_rows: Vec<Vec<&str>> = Vec::with_capacity(4);
    for row_index in 0..4 {
        let mut num_row: Vec<&str> = Vec::new();
        let mut start = 0;
        let mut raminos = reachers.iter();
        raminos.next();
        for end in raminos {
            let num = &rows[row_index][start..*end-1];
            num_row.push(num);
            // print!("{num},");
            start = *end;
        }
        // println!();
        num_rows.push(num_row);
    }
    
    let mut add_pointer = 0usize;
    let mut mult_pointer = 0usize;
    for (colp, op) in ops.iter().enumerate() {
        let flat_num_1 = num_rows[0][colp].as_bytes();
        let flat_num_2 = num_rows[1][colp].as_bytes();
        let flat_num_3 = num_rows[2][colp].as_bytes();
        let flat_num_4 = num_rows[3][colp].as_bytes();

        for flat_col in 0..4 {
            if flat_col == flat_num_1.len() {
                break;
            }
            let d1000 = if flat_num_1[flat_col] != b' ' {
                ((flat_num_1[flat_col] - b'0') as u64) * 1000u64
            } else {
                0
            };
            
            let d100 = if flat_num_2[flat_col] != b' ' {
                ((flat_num_2[flat_col] - b'0') as u64) * 100u64
            } else {
                0
            };
            
            let d10 = if flat_num_3[flat_col] != b' ' {
                ((flat_num_3[flat_col] - b'0') as u64) * 10u64
            } else {
                0
            };
             
            let d = if flat_num_4[flat_col] != b' ' {
                ((flat_num_4[flat_col] - b'0') as u64)
            } else {
                0
            };       


            let num = d1000 + d100 + d10 + d;


            if *op == b'+' {
                let v = add_pointer >> 6;
                let p = add_pointer & 63;
                unsafe {
                    add_rows
                        .get_unchecked_mut(flat_col)
                        .get_unchecked_mut(v)
                        .as_mut_array()[p] = num;
                }
                add_pointer += 1;
            } else {
                 let v = mult_pointer >> 6;
                let p = mult_pointer & 63;
                unsafe {
                    mult_rows
                        .get_unchecked_mut(flat_col)
                        .get_unchecked_mut(v)
                        .as_mut_array()[p] = num;
                }
                mult_pointer += 1;
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


