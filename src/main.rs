#![allow(unused)]
#![feature(coroutines, gen_blocks)]
#![feature(portable_simd)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    let input = include_str!("inputs/day07.txt");
    println!("{}", day07::part_2(input));
}
