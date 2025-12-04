#![allow(unused)]

use std::collections::HashSet;

const PRIMES: [u64; 5] = [1, 2, 3, 5, 7];

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
        }
    }

    println!("{sum}");
}

gen fn div_marcher(num: u64, div: u64, num_len: u64) -> u64 {
    let mut i = 1;
    let mut block = num;
    let mut block_len = num_len;
    let mut base = 10u64.pow((block_len - div) as u32);
    let divominator = 10u64.pow(div as u32);

    for i in 1..=(num_len / div) {
        let result = block / base;
        block = block % base;
        base /= divominator;
        yield result;
    }
}

gen fn next(num: u64, max: u64) -> u64 {
    let num_len = digits(num);
    for prime in (1..=(num / 2)) {
        // println!("- - ");
        // println!("prime = {prime}");
        let mut curr = num;
        if num_len % prime != 0 {
            // println!("Skipped becuz not divisible");
            continue;
        }
        if prime >= num_len {
            // println!("Breaking becuz my limits have been reached");
            break;
        }

        let mut divisions = div_marcher(curr, prime, num_len);
        let mut anchor = divisions.next().unwrap();
        // println!("initial anchor = {anchor}");
        for div in divisions {
            if div < anchor {
                break;
            } else if div > anchor {
                anchor += 1;
                break;
            }
        }
        // println!("anchor = {anchor}");

        loop {
            curr = repeat(anchor, num_len / prime, prime);
            if curr > max {
                break;
            }
            // println!("Next: {curr}");
            yield curr;
            anchor += 1;
        }
    }
}

pub fn part_2() {
    let input = include_str!("inputs/day02.txt");
    let mut gathered: HashSet<u64> = HashSet::with_capacity(2000);
    for (start, end) in get_ranges(input) {
        // println!("\n- - - -");
        // println!("Start: {start}, End: {end}");
        let start_len = digits(start);
        let end_len = digits(end);
        let carry = end_len - start_len;
        if carry > 0 {
            // println!("Carry: {carry}");
            let mid = 10u64.pow(start_len as u32);
            for num in next(start, mid - 1) {
                gathered.insert(num);
            }
            for num in next(mid, end) {
                gathered.insert(num);
            }
        } else {
            for num in next(start, end) {
                gathered.insert(num);
            }
        }

        // println!("End reached: {end}");
    }
    let sum: u64 = gathered.iter().sum();
    // right = 31898925685
    // right = 4174379265
    // println!("\n- - - -");
    // println!("Expected: 31898925685");
    // println!("Got:      {sum}");
    println!("{sum}");
}
