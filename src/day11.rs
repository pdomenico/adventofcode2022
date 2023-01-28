use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Add;

struct Monkey {
    items: RefCell<Vec<u32>>,
    operation: Box<dyn Fn(u32) -> u32>,
    test: Box<dyn Fn(u32) -> usize>,
    score: RefCell<u32>,
}

struct Monkeys(Vec<Monkey>);

impl Monkey {
    fn new(
        items: RefCell<Vec<u32>>,
        operation: Box<dyn Fn(u32) -> u32>,
        test: Box<dyn Fn(u32) -> usize>,
    ) -> Self {
        Monkey {
            items,
            operation,
            test,
            score: RefCell::new(0),
        }
    }

    fn add_item(&self, item: u32) {
        self.items.borrow_mut().push(item);
    }

    // fn turn(&mut self, monkeys: &mut Monkeys) {
    //     self.items.iter().for_each(|item| {
    //         self.score += 1;
    //         let new_val = (self.operation)(*item) / 3;
    //         let destination = (self.test)(new_val);
    //         monkeys.add_item_to_monkey(destination, *item);
    //     });

    //     self.items.clear();
    // }

    fn print(&self) {
        println!("Monkey items: {:?}", self.items);
        println!("Operation turns 10 into {}", (self.operation)(10));
        println!("Test returns {} from 10", (self.test)(10));
    }

    fn increment_score(&self) {
        self.score.replace_with(|x| *x + 1);
    }

    fn clear_items(&self) {
        self.items.borrow_mut().clear();
    }
}

impl Monkeys {
    fn new() -> Self {
        Monkeys(Vec::new())
    }

    fn add_monkey(&mut self, monkey: Monkey) {
        self.0.push(monkey);
    }

    fn round(&mut self) {
        for monkey in self.0.iter() {
            for item in monkey.items.borrow().iter() {
                monkey.increment_score();
                let new_val = (monkey.operation)(*item) / 3;
                let destination = (monkey.test)(new_val);
                self.add_item(destination, *item);
            }
            monkey.clear_items();
        }
    }

    fn add_item(&self, monkey: usize, item: u32) {
        self.0[monkey].add_item(item);
    }
}

pub fn part1() {
    let file = File::open("inputs/day11.txt").unwrap();
    let mut reader = io::BufReader::new(file).lines();

    let mut monkeys = Monkeys::new();

    while let Some(Ok(_)) = reader.next() {
        let items = reader
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .filter_map(|w| w.replace(",", "").parse::<u32>().ok())
            .collect::<Vec<u32>>();

        let operation_line = reader.next().unwrap().unwrap();
        let operation_vec = operation_line.split_whitespace().collect::<Vec<_>>();
        let (operator, operand) = (operation_vec[4], operation_vec[5]);

        // dbg!(operator, operand);

        let operation: Box<dyn Fn(u32) -> u32> = match (operator, operand.parse::<u32>()) {
            ("*", Ok(operand)) => Box::new(move |x| x * operand),
            ("+", Ok(operand)) => Box::new(move |x| x + operand),
            ("*", Err(_)) => Box::new(|x| x * x),
            ("+", Err(_)) => Box::new(|x| x + x),
            _ => panic!("Invalid operator"),
        };

        let test_value: u32 = reader
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let (monk_true, monk_false): (usize, usize) = (
            reader
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),
            reader
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),
        );

        let test = Box::new(move |x: u32| {
            if x % test_value == 0 {
                monk_true
            } else {
                monk_false
            }
        });

        monkeys.add_monkey(Monkey::new(RefCell::new(items), operation, test));
        reader.next();
    }

    for _ in 0..20 {
        monkeys.round()
    }

    let mut scores = monkeys
        .0
        .iter()
        .map(|monkey| monkey.score.borrow().clone())
        .collect::<Vec<_>>();
    scores.sort();
    scores.reverse();
    println!("Scores: {:?}", scores);
    println!("{}", scores[0] * scores[1]);
}
