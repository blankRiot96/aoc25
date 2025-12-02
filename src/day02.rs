#![allow(unused)]

use std::collections::HashSet;

fn parse_range(data: &str) -> (u64, u64) {
    let (start, end) = data.split_once("-").unwrap();
    (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
}

fn get_ranges(input: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    input.trim().split(",").map(parse_range)
}

fn digits(num: u64) -> u64 {
    (num.ilog10() + 1).into()
}

fn repeat(x: u64, n: u64, xlen: u64) -> u64 {
    let mut res = 0;
    let base = 10u64.pow(xlen as u32);
    for i in 0..n {
        let tenpow = base.pow(i as u32);
        res += x * tenpow;
    }
    return res;
}
//
// fn repeat(x: u64, n: u64, xlen: u64) -> u64 {
//     let mut res = 0;
//     let base = 10u64.pow(xlen as u32);
//     for _ in 0..n {
//         res = res * base + x;
//     }
//     res
// }

pub fn part_1() {
    let input = include_str!("inputs/day02.txt");
    let mut sum = 0u64;
    for (start, end) in get_ranges(input) {
        let mut num = start;
        while num <= end {
            let n_digits = digits(num);
            let half = n_digits / 2;
            if n_digits % 2 != 0 {
                num = 10u64.pow(n_digits as u32) + 10u64.pow(half as u32);
                continue;
            }
            
            let p = 10u64.pow(half as u32);
            let right = num % p;
            let left = num / p;
            if left == right {
                sum += num;
            } 
            let new = if right < left { left } else { left + 1 };
            let d = digits(new);
            num = repeat(new, 2, d);

            
            // println!("Next: {num}");
        }
    }

    println!("{sum}");
}

fn first_n_digits(num: u64, num_len: u64, first_how_many: u64) -> u64 {
    num / 10u64.pow((num_len - first_how_many) as u32)
}

pub fn part_2() {
    let input = include_str!("inputs/day02.txt");
    let mut sum = 0u64;
    let mut gathered: HashSet<u64> = HashSet::new();
    let mut carry = false;
    for (start, end) in get_ranges(input) {
        let mut n_digits = digits(start);
        if n_digits != digits(end) {
            carry = true;
        }
        'numloop: for num in start..=end {
            if carry {
                n_digits = digits(num);
            }

            for n in 1..=(n_digits / 2) {
                if repeat(first_n_digits(num, n_digits, n), n_digits / n, n) == num
                    && !gathered.contains(&num)
                {
                    sum += num;
                    gathered.insert(num);
                    continue 'numloop;
                }
            }
        }
        // println!("- - - -");
    }

    println!("{sum}");
}
