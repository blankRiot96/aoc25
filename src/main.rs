#![allow(unused)]
#![feature(coroutines, gen_blocks)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    let input = include_str!("inputs/day03.txt");
    println!("{}", day03::part_2(input));
}
