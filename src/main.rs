#![allow(unused)]
#![feature(coroutines, gen_blocks)]

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let input = include_bytes!("inputs/day04.txt");
    println!("{}", day04::part_1(input));
}
