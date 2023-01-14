use std::{fs::File, io, io::BufRead};

#[derive(Debug)]
struct Assignment {
    lower: i32,
    higher: i32,
}

pub fn part1() {
    let file = File::open("inputs/day4.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut count = 0;

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        let pairs: Vec<&str> = line.split(',').collect();
        let (pair1, pair2): (Vec<&str>, Vec<&str>) =
            (pairs[0].split('-').collect(), pairs[1].split('-').collect());
        let (assign1, assign2) = (
            Assignment {
                lower: pair1[0].parse().unwrap(),
                higher: pair1[1].parse().unwrap(),
            },
            Assignment {
                lower: pair2[0].parse().unwrap(),
                higher: pair2[1].parse().unwrap(),
            },
        );
        if (assign1.lower <= assign2.lower && assign1.higher >= assign2.higher)
            || (assign2.lower <= assign1.lower && assign2.higher >= assign1.higher)
        {
            count += 1;
        }
    });
    println!("{}", count);
}

pub fn part2() {
    let file = File::open("inputs/day4.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut count = 0;

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        let pairs: Vec<&str> = line.split(',').collect();
        let (pair1, pair2): (Vec<&str>, Vec<&str>) =
            (pairs[0].split('-').collect(), pairs[1].split('-').collect());
        let (assign1, assign2) = (
            Assignment {
                lower: pair1[0].parse().unwrap(),
                higher: pair1[1].parse().unwrap(),
            },
            Assignment {
                lower: pair2[0].parse().unwrap(),
                higher: pair2[1].parse().unwrap(),
            },
        );
        if (assign1.higher >= assign2.lower && assign1.lower <= assign2.lower)
            || (assign2.higher >= assign1.lower && assign2.lower <= assign1.lower)
        {
            count += 1;
        }
    });
    println!("{}", count);
}
