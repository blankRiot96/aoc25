use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::binary_heap;

pub fn part_1(input: &str) -> u64 {
    let mut sum = 0u64;
    for bank in input.lines() {
        let len = bank.len();
        let mut max = b'/';
        let mut maxi = 0;
        let mut smax = b'/';
        let mut smaxi = 0;
        for (i, &d) in bank.as_bytes().iter().enumerate() {
            if d > max {
                if i == len - 1 {
                    smax = max;
                    smaxi = maxi;
                } else {
                    smax = b'/';
                }
                max = d;
                maxi = i;
            } else if d > smax {
                smax = d;
                smaxi = i;
            }
        }

        if maxi < smaxi {
            let tens = (max - b'0') as u64 * 10;
            let units = (smax - b'0') as u64;
            sum += tens + units;
        } else {
            let tens = (smax - b'0') as u64 * 10;
            let units = (max - b'0') as u64;
            sum += tens + units;
        }
    }
    sum
}

pub fn part_2(input: &str) -> u128 {
    let mut total = 0u128;

    let expected: [u128; 4] = [987654321111, 811111111119, 434234234278, 888911112111];
    let mut gratzi = 0;
    for bank in input.lines() {
        println!("\n- - - -");
        println!("Bank: {bank}");
        let mut bank_digits = bank.as_bytes().iter();
        let mut digits: Vec<u8> = Vec::with_capacity(12);
        digits.push(*bank_digits.next().unwrap());

        for (i, &d) in bank_digits.enumerate() {
            if d > digits[0] {
                let placement = digits.len() - 1;
                digits[placement] = d;
                for _ in 0..(digits.len() - placement - 1) {
                    digits.pop();
                }
                // println!("{placement}");
            } else if digits.len() < 12 {
                digits.push(d);
            }
        }

        let num: u128 = digits.iter().rev().enumerate().fold(0, |sum, (i, &d)| {
            sum + ((d - b'0') as u128) * 10u128.pow(i as u32)
        });
        println!("Got:      {num}");
        println!("Expected: {}", expected[gratzi]);

        if num == expected[gratzi] {
            println!("");
        } else {
            println!("");
        }
        total += num;
        gratzi += 1;
    }

    total
}
