#![allow(unused)]

fn get_rotations(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.lines().map(|line| {
        line[1..].parse::<i32>().unwrap()
            * match line.chars().next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => panic!("Invalid instruction"),
            }
    })
}

pub fn part_1() {
    let input = include_str!("inputs/day01.txt");
    let mut answer = 0;
    get_rotations(input).fold(50, |mut dial, delta| {
        dial = (dial + delta).rem_euclid(100);
        if dial == 0 {
            answer += 1;
        }
        dial
    });
    println!("{answer}")
}

pub fn part_2() {
    let input = include_str!("inputs/day01.txt");
    let mut answer = 0;
    let mut dial = 50;

    for rotation in get_rotations(input) {
        let prev = dial;
        dial = (dial + rotation).rem_euclid(100);
        if dial == 0 {
            answer += 1;
        } else if (prev != 0) && ((rotation > 0 && prev > dial) || (rotation < 0 && prev < dial)) {
            answer += 1;
        }

        answer += rotation.abs() / 100;
    }

    println!("{answer}")
}
