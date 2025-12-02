#![allow(unused)]

use std::collections::HashSet;

fn parse_range(data: &str) -> (u64, u64) {
    let (start, end) = data.split_once("-").unwrap();
    (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
}

fn get_ranges(input: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    input.trim().split(",").map(parse_range)
}

pub fn part_1() {
    let input = include_str!("inputs/day02.txt");
    let mut sum = 0u64;
    for (start, end) in get_ranges(input) {
        for num in start..=end {
            let snum = num.to_string();
            let slen = snum.len();
            if slen % 2 != 0 {
                continue;
            }
            let (left, right) = snum.split_at(slen / 2);
            if left == right {
                sum += num;
            }
        }
    }

    println!("{sum}");
}

pub fn part_2() {
    let input = include_str!("inputs/day02.txt");
    let mut sum = 0u64;
    let mut gathered: HashSet<u64> = HashSet::new();
    for (start, end) in get_ranges(input) {
        'numloop: for num in start..=end {
            if gathered.contains(&num) {
                continue;
            }
            let snum = num.to_string();
            let slen = snum.len();
            for n in 1..=(slen / 2) {
                if (snum[..n].repeat(slen / n) == snum) {
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
