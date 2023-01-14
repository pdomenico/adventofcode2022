use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

pub fn part1() {
    let input = File::open("inputs/day6.txt").unwrap();
    let mut reader = io::BufReader::new(input).lines();

    let input_str = reader.next().unwrap().unwrap();

    let start = Instant::now();

    let res = input_str
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .position(|w| {
            let mut chars: [char; 4] = ['x'; 4];
            for (i, c) in w.iter().enumerate() {
                if chars.contains(c) {
                    return false;
                }
                chars[i] = *c;
            }
            true
        })
        .unwrap()
        + 4;

    println!("Took {:?}", start.elapsed());
    println!("{}", res);
}

// This is Ben Lichtman's algorithm
pub fn part2() {
    let input = File::open("inputs/day6.txt").unwrap();
    let mut reader = io::BufReader::new(input).lines();
    let input_str = reader.next().unwrap().unwrap();
    let start = Instant::now();

    // Initialize the state with the first 13 characters
    let mut state = 0u32;
    input_str
        .chars()
        .take(13)
        .for_each(|c| state ^= 1 << (c as u8 % 32));

    let res = input_str
        .chars()
        .collect::<Vec<_>>()
        .windows(14)
        .position(|window| {
            state ^= 1 << (window[13] as u8 % 32);
            if state.count_ones() == 14 {
                return true;
            }
            state ^= 1 << (window[0] as u8 % 32);
            false
        })
        .unwrap()
        + 14;

    println!("Took {:?}", start.elapsed());
    println!("{}", res);
}
