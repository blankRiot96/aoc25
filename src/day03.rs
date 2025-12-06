
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

fn get_num(digits: Vec<u8>) -> u128 {
    digits.iter().rev().enumerate().fold(0, |sum, (i, &d)| {
        sum + ((d - b'0') as u128) * 10u128.pow(i as u32)
    })
}

pub fn part_2(input: &str) -> u128 {
    let mut total = 0u128;

    for bank in input.lines() {
        let k = 12;
        let mut dro = bank.len() - k;
        let mut digits: Vec<u8> = Vec::new();
        for c in bank.as_bytes() {
            while dro > 0 && digits.len() > 0 && digits.last().unwrap() < c {
                digits.pop();
                dro -= 1;
            }
            digits.push(*c);
        }

        digits.truncate(k);
        let num = get_num(digits);
        total += num;
    }

    total
}
