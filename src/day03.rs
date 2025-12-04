
pub fn part_1(input: &str) -> u64 {
    let mut sum = 0u64;
    for bank in input.lines() {
        let len = bank.len();
        let mut bchars = bank.chars();
        let mut max = '/';
        let mut maxi = 0;
        let mut smax = '/';
        let mut smaxi = 0;
        for (i, d) in bchars.enumerate() {
            if d > max {
                if i == len - 1 {
                    smax = max;
                    smaxi = maxi;
                } else {
                    smax = '/';
                }
                max = d;
                maxi = i;
            } else if d > smax {
                smax = d;
                smaxi = i;
            }
        }

        if maxi < smaxi {
            let tens = ((max as u64) - 48) * 10;
            let units = (smax as u64) - 48;
            sum += tens + units;
            // println!("{}", tens + units);
        } else {
            // println!("{smax}");
            let tens = ((smax as u64) - 48) * 10;
            let units = (max as u64) - 48;
            sum += tens + units;
            // println!("{}", tens + units);
        }
    }
    sum
}
