use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn part1() {
    let file = File::open("inputs/day10.txt").unwrap();
    let mut reader = io::BufReader::new(file).lines();

    let mut reg_val = HashMap::new();
    reg_val.insert(0, 1);
    let mut current_cycle = 0;
    let mut last_register_val = 1;

    while let Some(Ok(line)) = reader.next() {
        if line.trim() == "noop" {
            current_cycle += 1;
            continue;
        }

        let n = line.split_whitespace().collect::<Vec<_>>()[1]
            .parse::<i32>()
            .unwrap();
        current_cycle += 2;
        last_register_val += n;
        // println!(
        //     "At cycle {} the register is {}",
        //     current_cycle, last_register_val
        // );
        reg_val.insert(current_cycle, last_register_val);
    }
    // println!("-----------------------------------------");

    let mut strength_sum = 0;
    for i in [20, 60, 100, 140, 180, 220] {
        let val = find_val(i - 1, &reg_val);
        // println!("At cycle {} the register is {}", i, val);
        let strength = val * i;
        // println!("Strength is {}", strength);
        strength_sum += strength;
    }

    println!("{}", strength_sum);
}

pub fn part2() {
    let file = File::open("inputs/day10.txt").unwrap();
    let mut reader = io::BufReader::new(file).lines();

    let mut reg_val = HashMap::new();
    reg_val.insert(0, 1);
    let mut current_cycle = 0;
    let mut last_register_val = 1;

    while let Some(Ok(line)) = reader.next() {
        if line.trim() == "noop" {
            current_cycle += 1;
            continue;
        }

        let n = line.split_whitespace().collect::<Vec<_>>()[1]
            .parse::<i32>()
            .unwrap();
        current_cycle += 2;
        last_register_val += n;
        reg_val.insert(current_cycle, last_register_val);
    }

    for row in 0..6 {
        for pixel in 0..40 {
            let cycle_n = row * 40 + pixel;

            let val = find_val(cycle_n, &reg_val);

            if (pixel - val).abs() < 2 {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}

fn find_val(cycle: i32, reg_val: &HashMap<i32, i32>) -> i32 {
    match reg_val.get(&cycle) {
        Some(val) => *val,
        None => find_val(cycle - 1, reg_val),
    }
}
