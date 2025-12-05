use std::ops::RangeInclusive;

gen fn get_fresh_ranges(raw: &str) -> RangeInclusive<u64> {
    for line in raw.lines() {
        let (start, end) = line.split_once("-").unwrap();
        yield (start.parse::<u64>().unwrap())..=(end.parse::<u64>().unwrap());
    }
}

gen fn parse_ingredients(raw: &str) -> u64 {
    for line in raw.lines() {
        yield line.parse::<u64>().unwrap();
    }
}

pub fn part_1(input: &str) -> u16 {
    let (range_raw, ing_raw) = input.split_once("\n\n").unwrap();
    let mut ranges_vec = get_fresh_ranges(range_raw).collect::<Vec<RangeInclusive<u64>>>();
    let mut ingredients_vec = parse_ingredients(ing_raw).collect::<Vec<u64>>();

    ranges_vec.sort_by_key(|r| *r.start());
    ingredients_vec.sort();

    let mut ranges = ranges_vec.iter();
    let mut ingredients = ingredients_vec.iter();

    let mut range = ranges.next().unwrap();
    let mut ing = ingredients.next().unwrap();
    let mut fresh = 0u16;

    loop {
        println!("\n- - - -");
        println!("Current Range: [{} - {}]", range.start(), range.end());
        println!("Current Item:   {ing}");
        if ing < range.start() {
            println!("{ing} is spoilt");
            ing = ingredients.next().unwrap();
            continue;
        } else if ing > range.end() {
            println!("{ing} is greater than {}", range.end());
            range = match ranges.next() {
                Some(r) => r,
                None => break,
            };
            continue;
        } else {
            fresh += 1;
            println!("{ing} is fresh");
            ing = match ingredients.next() {
                Some(i) => i,
                None => break,
            };
        }
    }

    fresh
}

fn dbg_range(range: &RangeInclusive<u64>) -> String {
    format!("\n{}\n{}", range.start(), range.end())
}

pub fn part_2(input: &str) -> u64 {
    let (range_raw, _) = input.split_once("\n\n").unwrap();
    let mut ranges_vec = get_fresh_ranges(range_raw).collect::<Vec<RangeInclusive<u64>>>();
    ranges_vec.sort_by_key(|r| *r.start());
    let mut ranges = ranges_vec.iter();
    let r = ranges.next().unwrap();
    let mut total = r.end() - r.start() + 1;
    let mut greatest = r.end();

    println!("First range: {}", dbg_range(&r));
    for range in ranges {
        println!("- - - -");
        println!("Range selected: {}", dbg_range(&range));
        if range.start() <= greatest {
            println!("range.start < prev.end");
            if range.end() <= greatest {
                println!("range completely smaller than prev");
                continue;
            }
            total += range.end() - greatest;
            greatest = range.end();
            continue;
        }
        println!("fresh range");
        total += range.end() - range.start() + 1;
        greatest = range.end();
    }

    total
}
