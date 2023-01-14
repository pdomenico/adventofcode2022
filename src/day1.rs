use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::ops::Add;

pub fn day1() {
    let input = File::open("inputs/day1.txt").unwrap();
    let reader = BufReader::new(input).lines();
    let mut elves = vec![0];

    for line in reader {
        if let Ok(line) = line {
            if let Ok(n) = line.trim().parse::<i32>() {
                let last = elves.len() - 1;
                elves[last] += n;
                continue;
            }
            elves.push(0);
        }
    }
    elves.sort();
    elves.reverse();
    println!("{}", elves[0]);
    println!("{}", elves[0] + elves[1] + elves[2]);
}
