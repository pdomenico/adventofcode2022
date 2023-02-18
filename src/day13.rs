use std::{cmp::Ordering, fs::File, io, io::BufRead};

#[derive(Debug, Clone)]
enum Packet {
    List(Vec<Box<Packet>>),
    Number(u32),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match compare_packets(self, other) {
            Some(true) => Ordering::Less,
            Some(false) => Ordering::Greater,
            None => Ordering::Equal,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match compare_packets(self, other) {
            Some(_) => false,
            None => true,
        }
    }
}

impl Eq for Packet {}

fn print_packet(packet: &Packet) {
    match packet {
        Packet::Number(n) => print!("{n}"),
        Packet::List(list) => {
            if list.len() == 0 {
                print!("[]");
                return;
            }
            print!("[");
            for (i, element) in list.iter().enumerate() {
                if i == list.len() - 1 {
                    print_packet(element);
                    print!("]");
                } else {
                    print_packet(element);
                    print!(",");
                }
            }
        }
    }
}

fn parse_packet(s: &str) -> Packet {
    if s == "[]" {
        return Packet::List(Vec::new());
    }

    if let Ok(n) = s.parse::<u32>() {
        return Packet::Number(n);
    }

    let mut chars = s.chars();
    let mut buffer = Vec::new();
    let mut elements = Vec::new();
    while let Some(c) = chars.next() {
        match c {
            '[' => {
                let mut paren_count = 1;
                while let Some(c) = chars.next() {
                    match c {
                        ']' => {
                            paren_count -= 1;
                            if paren_count > 0 {
                                buffer.push(c);
                            }
                        }
                        '[' => {
                            buffer.push(c);
                            paren_count += 1;
                        }
                        _ => buffer.push(c),
                    }
                    if paren_count == 0 {
                        break;
                    }
                }
                if buffer.len() == 0 {
                    elements.push(Packet::List(Vec::new()));
                } else {
                    elements.push(parse_packet(
                        buffer.into_iter().collect::<String>().as_str(),
                    ));
                    buffer = Vec::new();
                }
            }
            ',' => {
                if buffer.len() != 0 {
                    elements.push(parse_packet(
                        buffer.into_iter().collect::<String>().as_str(),
                    ));
                    buffer = Vec::new();
                }
            }
            _ => buffer.push(c),
        }
    }
    if buffer.len() != 0 {
        elements.push(parse_packet(
            buffer.into_iter().collect::<String>().as_str(),
        ));
    }

    Packet::List(elements.into_iter().map(Box::new).collect())
}

fn compare_packets(p1: &Packet, p2: &Packet) -> Option<bool> {
    match (p1, p2) {
        (Packet::Number(a), Packet::Number(b)) => match a.cmp(&b) {
            Ordering::Less => Some(true),
            Ordering::Greater => Some(false),
            Ordering::Equal => None,
        },
        (Packet::Number(a), Packet::List(list)) => compare_packets(
            &Packet::List(vec![Box::new(Packet::Number(*a))]),
            &Packet::List(list.to_vec()),
        ),
        (Packet::List(list), Packet::Number(b)) => compare_packets(
            &Packet::List(list.to_vec()),
            &Packet::List(vec![Box::new(Packet::Number(*b))]),
        ),
        (Packet::List(list1), Packet::List(list2)) => {
            let mut res = None;
            let (mut list1_iter, mut list2_iter) = (list1.iter(), list2.iter());
            while let None = res {
                res = match (list1_iter.next(), list2_iter.next()) {
                    (Some(a), Some(b)) => compare_packets(a, b),
                    (Some(_), None) => Some(false),
                    (None, Some(_)) => Some(true),
                    (None, None) => break,
                }
            }
            res
        }
    }
}

fn read_input(filepath: &str) -> Vec<(Packet, Packet)> {
    let file = File::open(filepath).unwrap();
    let mut reader = io::BufReader::new(file).lines();
    let mut packets = Vec::new();

    while let Some(Ok(line1)) = reader.next() {
        let line2 = reader.next().unwrap().unwrap();

        packets.push((
            parse_packet(&line1.as_str()[1..(line1.len() - 1)]),
            parse_packet(&line2.as_str()[1..(line2.len() - 1)]),
        ));
        reader.next();
    }

    packets
}

pub fn part1() {
    let packets = read_input("inputs/day13.txt");

    let result = packets
        .into_iter()
        .enumerate()
        .filter_map(|(i, (p1, p2))| {
            if compare_packets(&p1, &p2).unwrap() {
                Some(i + 1)
            } else {
                None
            }
        })
        .fold(0, |a, b| a + b);

    println!("{}", result);
}

pub fn part2() {
    let packets_couples = read_input("inputs/day13.txt");
    let mut packets = Vec::new();

    for (p1, p2) in packets_couples.into_iter() {
        packets.push(p1);
        packets.push(p2);
    }

    let (div1, div2) = (parse_packet("[2]"), parse_packet("[6]"));
    packets.push(div1.clone());
    packets.push(div2.clone());

    packets.sort();
    let position1 = packets.iter().position(|e| *e == div1).unwrap();
    let position2 = packets.iter().position(|e| *e == div2).unwrap();
    println!("{}", (position1 + 1) * (position2 + 1));
}
