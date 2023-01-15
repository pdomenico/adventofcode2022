use std::fs::File;
use std::io::{self, BufRead};

pub fn part1() {
    let grid = get_grid();

    // let mut visible = (grid.len() * 2) + (grid[0].len() * 2) - 4;
    let mut visible = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let height = grid[i][j];
            let (mut left, mut right, mut up, mut down) = (true, true, true, true);

            // Left side
            for k in 0..j {
                if grid[i][k] >= height {
                    left = false;
                }
            }

            // right
            for k in (j + 1)..grid[i].len() {
                if grid[i][k] >= height {
                    right = false;
                }
            }

            // up
            for k in 0..i {
                if grid[k][j] >= height {
                    up = false;
                }
            }

            // down
            for k in (i + 1)..grid.len() {
                if grid[k][j] >= height {
                    down = false;
                }
            }

            if left || right || up || down {
                visible += 1;
            }
        }
    }

    println!("{}", visible);
}

pub fn part2() {
    let grid = get_grid();

    let mut highest_score = 0;

    for (i, row) in grid.iter().enumerate() {
        if i == 0 || i == grid.len() - 1 {
            continue;
        }

        for (j, _) in row.iter().enumerate() {
            if j == 0 || j == row.len() - 1 {
                continue;
            }

            // look left
            let mut left = 0;
            for k in (0..j).rev() {
                left += 1;
                if grid[i][k] >= grid[i][j] {
                    break;
                }
            }

            let mut right = 0;
            for k in (j + 1)..grid[i].len() {
                right += 1;
                if grid[i][k] >= grid[i][j] {
                    break;
                }
            }

            let mut up = 0;
            for k in (0..i).rev() {
                up += 1;
                if grid[k][j] >= grid[i][j] {
                    break;
                }
            }

            let mut down = 0;
            for k in (i + 1)..grid.len() {
                down += 1;
                if grid[k][j] >= grid[i][j] {
                    break;
                }
            }

            if i == 2 && j == 4 {
                println!("Tree height: {}", grid[i][j]);
                println!("left: {}", left);
                println!("right: {}", right);
                println!("up: {}", up);
                println!("down: {}", down);
            }

            let score = left * right * up * down;
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    println!("{}", highest_score);
}

fn get_grid() -> Vec<Vec<i32>> {
    let input = io::BufReader::new(File::open("inputs/day8.txt").unwrap()).lines();

    input
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
