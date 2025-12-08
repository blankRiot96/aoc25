use std::simd::{u8x64, u64x64};
use std::simd::prelude::{SimdPartialEq, SimdPartialOrd};

const TWO_SPLAT: u8x64 = u8x64::splat(2);

fn find_2(beam: u8x64) -> Option<usize> {
    let mask = beam.simd_eq(TWO_SPLAT);
    let bits = mask.to_bitmask();
    if bits == 0 {
        return None;
    }

    Some(bits.trailing_zeros() as usize)
}

fn resolve(beam: &mut Vec<u8x64>) {
    for chunk_index in 0..beam.len() {
        loop {
            match find_2(beam[chunk_index]) {
                Some(index) => {
                    beam[chunk_index][index] = 0;
                    if index > 0 {
                        beam[chunk_index][index - 1] = 1;
                    } else if chunk_index > 0 {
                        beam[chunk_index - 1][63] = 1;
                    }

                    if index < 63 {
                        beam[chunk_index][index + 1] = 1;
                    } else if chunk_index < 2 {
                        beam[chunk_index + 1][0] = 1;
                    }
                },
                None => break,
            } 
        }
    }
}

fn to_split_mask(line_chunk: &str) -> u8x64 {
    let b = line_chunk.as_bytes();
    let mut buf = u8x64::splat(0);
    for i in 0..b.len() {
        if b[i] == b'^' {
            buf[i] = 1;
        } else {
            buf[i] = 0;
        }
    }
    
    buf
}


fn count_2s(beam: u8x64) -> u32 {
    let mask = beam.simd_eq(TWO_SPLAT);
    mask.to_bitmask().count_ones()
}

pub fn part_1(input: &str) -> u32 {
    let mut n_splits = 0u32;
    let mut beam = vec![
        u8x64::splat(0),
        u8x64::splat(0),
        u8x64::splat(0),
    ];
    beam[1][6] = 1;
    let mut lines = input.lines();
    lines.next();
    lines.next();
    while let Some(line) = lines.next() {
        println!("Line:      {line}");
        let splitters = vec![
            to_split_mask(&line[..64]) & beam[0],
            to_split_mask(&line[64..128]) & beam[1],
            to_split_mask(&line[128..]) & beam[2],
        ];
        println!("PreBeam:   {beam:?}");
        println!("Splitters: {splitters:?}");
        
        for (i, splitter) in splitters.iter().enumerate() {
            beam[i] += splitter;
            n_splits += count_2s(beam[i]);
        }
        println!("Beam:      {beam:?}");
        resolve(&mut beam); 
        lines.next();
    }
    
    println!("\n- - - -");
    println!("Expected: 1537");
    println!("Got:      {n_splits}");
    n_splits
}


pub fn part_2(input: &str) -> u64 {
    let mut timelines = vec![
        u64x64::splat(0),
        u64x64::splat(0),
        u64x64::splat(0),
    ];

    timelines[1][6] = 1;

    let mut lines = input.lines();
    lines.next();
    lines.next();

    while let Some(line) = lines.next() {
        let splitter_masks = vec![
            to_split_mask(&line[..64]),
            to_split_mask(&line[64..128]),
            to_split_mask(&line[128..]),
        ];

        let mut next_timelines = vec![
            u64x64::splat(0),
            u64x64::splat(0),
            u64x64::splat(0),
        ];

        for chunk_index in 0..3 {
            let splitter_chunk = splitter_masks[chunk_index];

            for i in 0..64 {
                let count = timelines[chunk_index][i];
                if count == 0 {
                    continue;
                }

                if splitter_chunk[i] == 1 {
                    if i > 0 {
                        next_timelines[chunk_index][i - 1] += count;
                    } else if chunk_index > 0 {
                        next_timelines[chunk_index - 1][63] += count;
                    }

                    if i < 63 {
                        next_timelines[chunk_index][i + 1] += count;
                    } else if chunk_index < 2 {
                        next_timelines[chunk_index + 1][0] += count;
                    }
                } else {
                    next_timelines[chunk_index][i] += count;
                }
            }
        }

        timelines = next_timelines;

        lines.next();
    }

    let mut total = 0u64;
    for chunk_index in 0..3 {
        for i in 0..64 {
            total += timelines[chunk_index][i];
        }
    }

    total
}
