use std::{fs::File, io, io::BufRead};

pub fn part1() {
    let input = File::open("inputs/day5.txt").unwrap();
    let mut reader = io::BufReader::new(input).lines().into_iter();
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(9);
    let mut stack_finished = false;

    while let Some(Ok(line)) = reader.next() {
        if !stack_finished {
            let mut chars = line.chars();
            let mut stack_count = 0;
            while let Some(c) = chars.next() {
                match c {
                    '[' => {
                        match stacks.iter_mut().nth(stack_count) {
                            Some(stack) => stack.push(chars.next().unwrap()),
                            None => {
                                while stacks.len() < stack_count + 1 {
                                    stacks.push(Vec::new());
                                }
                                stacks.last_mut().unwrap().push(chars.next().unwrap());
                            }
                        }
                        chars.next();
                        chars.next();
                        stack_count += 1;
                    }
                    _ => {
                        chars.next();
                        chars.next();
                        chars.next();
                        stack_count += 1;
                    }
                }
            }
            if stacks.iter().any(|stack| stack.len() == 8) {
                stack_finished = true;
                stacks.iter_mut().for_each(|stack| stack.reverse());
                reader.next();
                reader.next();
                for (i, stack) in stacks.iter().enumerate() {
                    println!("{i}: {stack:?}");
                }
            }
            continue;
        }
        let data: Vec<&str> = line.split_whitespace().collect();
        let (n, from, to): (usize, usize, usize) = (
            data[1].parse().unwrap(),
            data[3].parse().unwrap(),
            data[5].parse().unwrap(),
        );
        for _ in 0..n {
            let tmp = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(tmp);
        }
    }

    stacks
        .iter()
        .for_each(|stack| print!("{}", stack.last().unwrap()));
    println!("");
}

pub fn part2() {
    let input = File::open("inputs/day5.txt").unwrap();
    let mut reader = io::BufReader::new(input).lines().into_iter();
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(9);
    let mut stack_finished = false;

    while let Some(Ok(line)) = reader.next() {
        if !stack_finished {
            let mut chars = line.chars();
            let mut stack_count = 0;
            while let Some(c) = chars.next() {
                match c {
                    '[' => {
                        match stacks.iter_mut().nth(stack_count) {
                            Some(stack) => stack.push(chars.next().unwrap()),
                            None => {
                                while stacks.len() < stack_count + 1 {
                                    stacks.push(Vec::new());
                                }
                                stacks.last_mut().unwrap().push(chars.next().unwrap());
                            }
                        }
                        chars.next();
                        chars.next();
                        stack_count += 1;
                    }
                    _ => {
                        chars.next();
                        chars.next();
                        chars.next();
                        stack_count += 1;
                    }
                }
            }
            if stacks.iter().any(|stack| stack.len() == 8) {
                stack_finished = true;
                stacks.iter_mut().for_each(|stack| stack.reverse());
                reader.next();
                reader.next();
                for (i, stack) in stacks.iter().enumerate() {
                    println!("{i}: {stack:?}");
                }
            }
            continue;
        }
        let data: Vec<&str> = line.split_whitespace().collect();
        let (n, from, to): (usize, usize, usize) = (
            data[1].parse().unwrap(),
            data[3].parse().unwrap(),
            data[5].parse().unwrap(),
        );
        let mut tmp = Vec::new();
        for _ in 0..n {
            tmp.push(stacks[from - 1].pop().unwrap());
        }
        tmp.reverse();
        stacks[to - 1].append(&mut tmp);
    }

    stacks
        .iter()
        .for_each(|stack| print!("{}", stack.last().unwrap()));
    println!("");
}
