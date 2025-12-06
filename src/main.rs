#![allow(unused)]
#![feature(coroutines, gen_blocks)]
#![feature(portable_simd)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let input = include_str!("inputs/day06.txt");
    println!("{}", day06::part_1(input));
}
