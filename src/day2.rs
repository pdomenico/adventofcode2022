use std::{fs::File, io, io::BufRead};

enum Choice {
    Rock,
    Paper,
    Scissor,
}

pub fn day2() {
    let input = File::open("inputs/day2.txt").unwrap();
    let lines = io::BufReader::new(input).lines();
    let mut total_score = 0;
    lines.for_each(|line| {
        // if let Ok(line) = line {
        //     let choices: Vec<&str> = line.trim().split_whitespace().collect();
        //     let (choice_a, choice_b) = (letter_to_choice(choices[0]), letter_to_choice(choices[1]));
        //     total_score += calculate_score(choice_b, choice_a);
        // }

        if let Ok(line) = line {
            let data: Vec<&str> = line.trim().split_whitespace().collect();
            let (adv_choice, end) = (letter_to_choice(data[0]), data[1]);
            total_score += calculate_score(&compute_choice(&adv_choice, end), &adv_choice);
        }
    });
    println!("{total_score}");
}

fn letter_to_choice(letter: &str) -> Choice {
    match letter {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissor,
        _ => unreachable!(),
    }
}

fn compute_choice(adv: &Choice, end: &str) -> Choice {
    match (adv, end) {
        (Choice::Rock, "X") => Choice::Scissor,
        (Choice::Rock, "Y") => Choice::Rock,
        (Choice::Rock, "Z") => Choice::Paper,
        (Choice::Paper, "X") => Choice::Rock,
        (Choice::Paper, "Y") => Choice::Paper,
        (Choice::Paper, "Z") => Choice::Scissor,
        (Choice::Scissor, "X") => Choice::Paper,
        (Choice::Scissor, "Y") => Choice::Scissor,
        (Choice::Scissor, "Z") => Choice::Rock,
        _ => unreachable!(),
    }
}

fn calculate_score(me: &Choice, adv: &Choice) -> i32 {
    match (me, adv) {
        (Choice::Rock, Choice::Rock) => 1 + 3,
        (Choice::Rock, Choice::Paper) => 1,
        (Choice::Rock, Choice::Scissor) => 1 + 6,
        (Choice::Paper, Choice::Rock) => 6 + 2,
        (Choice::Paper, Choice::Paper) => 2 + 3,
        (Choice::Paper, Choice::Scissor) => 2 + 0,
        (Choice::Scissor, Choice::Rock) => 3 + 0,
        (Choice::Scissor, Choice::Paper) => 3 + 6,
        (Choice::Scissor, Choice::Scissor) => 3 + 3,
    }
}
