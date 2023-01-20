use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Hash, Eq, PartialEq)]
struct Coordinates {
    x: i32,
    y: i32,
}

enum Move {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

struct Board {
    head: Coordinates,
    tails: Vec<Coordinates>,
    last_tail_positions: HashSet<Coordinates>,
}

impl Board {
    fn new() -> Self {
        Board {
            head: Coordinates { x: 0, y: 0 },
            tails: {
                let mut tails = Vec::new();
                for _ in 0..9 {
                    tails.push(Coordinates { x: 0, y: 0 });
                }
                tails
            },
            last_tail_positions: {
                let mut set = HashSet::new();
                set.insert(Coordinates { x: 0, y: 0 });
                set
            },
        }
    }

    fn move_head(&mut self, m: Move) {
        match m {
            Move::Up(n) => {
                for _ in 0..n {
                    self.head.y += 1;
                    self.adjust_tails();
                }
            }
            Move::Down(n) => {
                for _ in 0..n {
                    self.head.y -= 1;
                    self.adjust_tails();
                }
            }
            Move::Left(n) => {
                for _ in 0..n {
                    self.head.x -= 1;
                    self.adjust_tails();
                }
            }
            Move::Right(n) => {
                for _ in 0..n {
                    self.head.x += 1;
                    self.adjust_tails();
                }
            }
        }
    }

    fn adjust_tails(&mut self) {
        let mut prev_coord = self.head.clone();
        for i in 0..9 {
            let curr_coord = self.tails[i].clone();
            if are_touching(&prev_coord, &curr_coord) {
                break;
            }
            if prev_coord.x > curr_coord.x {
                self.tails[i].x += 1;
            }
            if prev_coord.x < curr_coord.x {
                self.tails[i].x -= 1;
            }
            if prev_coord.y > curr_coord.y {
                self.tails[i].y += 1;
            }
            if prev_coord.y < curr_coord.y {
                self.tails[i].y -= 1;
            }

            prev_coord = self.tails[i].clone();
        }
        self.last_tail_positions.insert(self.tails[8].clone());
    }
}

fn are_touching(c1: &Coordinates, c2: &Coordinates) -> bool {
    (c1.x - c2.x).abs() < 2 && (c1.y - c2.y).abs() < 2
}

pub fn part2() {
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

    println!("Part 2: {}", board.last_tail_positions.len());
}
