use std::simd::u64x64;
use std::simd::num::SimdUint;

pub fn part_1(input: &str) -> u64 {
    let guh = input.lines().last().unwrap();
    let mut ops = Vec::new();
    for op in guh.split(" ") {
        if op.len() > 0 {
            ops.push(op);
        }
    }


    // let n_vecs = ((ops.len() / 64) + 1);
    let mut add_rows: Vec<Vec<u64x64>> = Vec::with_capacity(4);
    let mut mult_rows: Vec<Vec<u64x64>> = Vec::with_capacity(4);
    
    for (row_index, row) in input.lines().take(4).enumerate() {
        add_rows.push(Vec::new());
        mult_rows.push(Vec::new());

        let mut col_pointer = 0;
        let mut add_pointer = 0;
        let mut mult_pointer = 0;
        for num in row.split(" ") {
            if num.len() == 0 {
                continue;
            }

            let n = num.parse::<u64>().unwrap();
            
            if ops[col_pointer] == "+" {
                if add_pointer % 64 == 0 {
                    add_rows[row_index].push(u64x64::splat(0));
                }
                let add_vec_pointer = add_pointer / 64;
                add_rows[row_index][add_vec_pointer][add_pointer % 64] = n;
                add_pointer += 1;
            } else {
                if mult_pointer % 64 == 0 {
                    mult_rows[row_index].push(u64x64::splat(0));
                }
                let mult_vec_pointer = mult_pointer / 64;
                mult_rows[row_index][mult_vec_pointer][mult_pointer % 64] = n;
                mult_pointer += 1;
            }

            col_pointer += 1;
        }
    }

    let mut total = 0;

    for vi in 0..(add_rows[0].len()) {
        let res_add = add_rows[0][vi] + add_rows[1][vi] + add_rows[2][vi] + add_rows[3][vi];
        total += res_add.reduce_sum();
    }
    
    for vi in 0..(mult_rows[0].len()) {
        let res_mult = mult_rows[0][vi] * mult_rows[1][vi] * mult_rows[2][vi] * mult_rows[3][vi];
        total += res_mult.reduce_sum();
    }
    total
}
