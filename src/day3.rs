use std::{fs::File, io, io::BufRead};

pub fn part1() {
    let input = File::open("inputs/day3.txt").unwrap();
    let lines = io::BufReader::new(input).lines();
    let mut score = 0;

    for line in lines {
        let line = line.unwrap();
        let (comp1, comp2) = line.split_at(line.len() / 2);
        println!("{} {}", comp1, comp2);
        let comp1_vec: Vec<char> = comp1.chars().collect();
        let common = comp2.chars().find(|c| comp1_vec.contains(c)).unwrap();
        score += get_score(common);
    }
    println!("{}", score);
}

pub fn part2() {
    let input = File::open("inputs/day3.txt").unwrap();
    let mut score = 0;
    let mut lines = io::BufReader::new(input).lines();

    while let (Some(Ok(line1)), Some(Ok(line2)), Some(Ok(line3))) =
        (lines.next(), lines.next(), lines.next())
    {
        let c = line1
            .chars()
            .filter(|c| line2.chars().collect::<Vec<char>>().contains(c))
            .find(|c| line3.chars().collect::<Vec<char>>().contains(c));
        score += get_score(c.unwrap());
    }
    println!("{score}");
}
fn get_score(letter: char) -> i32 {
    let value: u8;
    let mut score: i32 = 0;
    if letter.is_uppercase() {
        value = (letter as u8) + 32;
        score += 26;
    } else {
        value = letter as u8;
    }
    score += (value as i32) - 96;
    score
}
