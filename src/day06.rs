
use std::simd::num::SimdUint;
use std::simd::u64x64;

#[inline(always)]
fn fastu64(s: &str) -> u64 {
    let mut x = 0u64;
    for &b in s.as_bytes() {
        x = x * 10 + (b - b'0') as u64;
    }
    x
}

pub fn part_1(input: &str) -> u64 {
    let ops: Vec<u8> = input
        .lines()
        .last()
        .unwrap()
        .as_bytes()
        .iter()
        .copied()
        .filter(|&b| b != b' ')
        .collect();
    // ops.reverse();

    let rows: Vec<&str> = input.lines().take(4).collect();
    let w = ops.len();
    let n_vecs = (w + 63) / 64;

    let mut add_rows: Vec<Vec<u64x64>> = vec![vec![u64x64::splat(0); n_vecs]; 4];
    let mut mult_rows: Vec<Vec<u64x64>> = vec![vec![u64x64::splat(0); n_vecs]; 4];

    for row_index in 0..4 {
        let mut add_pointer = 0usize;
        let mut mult_pointer = 0usize;

        for (i, num) in rows[row_index].split_ascii_whitespace().enumerate() {
            let n = fastu64(num);
            let op = ops[i];

            if op == b'+' {
                let v = add_pointer >> 6;
                let p = add_pointer & 63;
                add_rows[row_index][v][p] = n;
                add_pointer += 1;
            } else {
                let v = mult_pointer >> 6;
                let p = mult_pointer & 63;
                mult_rows[row_index][v][p] = n;
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

