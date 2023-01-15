use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
}

struct Board {
    head: Coordinates,
    tail: Coordinates,
    positions: HashSet<Coordinates>,
}

enum Move {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl Board {
    fn new() -> Self {
        Board {
            head: Coordinates { x: 0, y: 0 },
            tail: Coordinates { x: 0, y: 0 },
            positions: {
                let mut set = HashSet::new();
                set.insert(Coordinates { x: 0, y: 0 });
                set
            },
        }
    }

    fn touching(&self) -> bool {
        (self.head.x - self.tail.x).abs() < 2 && (self.head.y - self.tail.y).abs() < 2
    }

    fn move_head(&mut self, m: Move) {
        match m {
            Move::Up(n) => {
                for _ in 0..n {
                    self.head.y += 1;
                    self.adjust_tail();
                }
            }
            Move::Down(n) => {
                for _ in 0..n {
                    self.head.y -= 1;
                    self.adjust_tail();
                }
            }
            Move::Left(n) => {
                for _ in 0..n {
                    self.head.x -= 1;
                    self.adjust_tail();
                }
            }
            Move::Right(n) => {
                for _ in 0..n {
                    self.head.x += 1;
                    self.adjust_tail();
                }
            }
        }
    }

    fn adjust_tail(&mut self) {
        if self.touching() {
            return;
        }

        if self.head.x > self.tail.x {
            self.tail.x += 1;
        }
        if self.head.x < self.tail.x {
            self.tail.x -= 1;
        }
        if self.head.y > self.tail.y {
            self.tail.y += 1;
        }
        if self.head.y < self.tail.y {
            self.tail.y -= 1;
        }

        self.positions.insert(self.tail.clone());
    }
}

pub fn part1() {
    let mut input = io::BufReader::new(File::open("inputs/day9.txt").unwrap()).lines();

    let mut board = Board::new();

    while let Some(Ok(line)) = input.next() {
        let data = line.split_whitespace().collect::<Vec<_>>();
        match data[0] {
            "U" => {
                let n = data[1].parse::<u32>().unwrap();
                board.move_head(Move::Up(n));
            }
            "D" => {
                let n = data[1].parse::<u32>().unwrap();
                board.move_head(Move::Down(n));
            }
            "R" => {
                let n = data[1].parse::<u32>().unwrap();
                board.move_head(Move::Right(n));
            }
            "L" => {
                let n = data[1].parse::<u32>().unwrap();
                board.move_head(Move::Left(n));
            }
            _ => panic!("Unknown direction"),
        }
    }

    println!("Part 1: {}", board.positions.len());
}
