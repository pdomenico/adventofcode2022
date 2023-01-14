use std::cell::RefCell;
use std::fs;
use std::io::{self, BufRead};
use std::rc::Rc;

struct Directory<'a> {
    name: &'a str,
    files: RefCell<Vec<File<'a>>>,
    subdirs: RefCell<Vec<Rc<Directory<'a>>>>,
    parent: Option<Rc<Directory<'a>>>,
}

struct File<'a> {
    name: &'a str,
    size: usize,
}

impl<'a> Directory<'a> {
    fn size(&self) -> usize {
        let mut size = 0;
        for file in self.files.borrow().iter() {
            size += file.size;
        }
        for subdir in self.subdirs.borrow().iter() {
            size += subdir.size();
        }
        size
    }

    fn add_file(&self, filename: &'a str, size: usize) {
        self.files.borrow_mut().push(File {
            name: filename,
            size,
        });
    }

    fn add_subdir(self: Rc<Directory<'a>>, name: &'a str) {
        self.subdirs.borrow_mut().push(Rc::new(Directory {
            name,
            files: RefCell::new(Vec::new()),
            parent: Some(self.clone()),
            subdirs: RefCell::new(Vec::new()),
        }))
    }

    fn get_parent(&self) -> Option<Rc<Directory<'a>>> {
        match &self.parent {
            Some(parent) => Some(parent.clone()),
            None => None,
        }
    }

    fn get_subdir(&self, name: &str) -> Option<Rc<Directory<'a>>> {
        match self.subdirs.borrow().iter().find(|dir| dir.name == name) {
            Some(dir) => Some(dir.clone()),
            None => None,
        }
    }
}

pub fn main() {
    let input = fs::File::open("inputs/day7.txt").unwrap();
    let reader = io::BufReader::new(input).lines();
    let lines: Vec<String> = reader.map(|line| line.unwrap()).collect();

    let root_dir = Rc::new(Directory {
        name: "/",
        files: RefCell::new(Vec::new()),
        subdirs: RefCell::new(Vec::new()),
        parent: None,
    });

    let mut current_dir = root_dir.clone();

    let mut i = 1;

    while i < lines.len() {
        if lines[i] == "$ ls" {
            i += 1;
            while i < lines.len() && !lines[i].starts_with("$") {
                if lines[i].starts_with("dir") {
                    current_dir
                        .clone()
                        .add_subdir(lines[i].split_whitespace().collect::<Vec<_>>()[1]);
                } else {
                    let data = lines[i].split_whitespace().collect::<Vec<_>>();
                    current_dir.add_file(data[1], data[0].parse().unwrap());
                }
                i += 1;
            }
            continue;
        }

        if lines[i] == "$ cd .." {
            match current_dir.get_parent() {
                Some(parent) => current_dir = parent,
                None => (),
            }
            i += 1;
            continue;
        }

        let new_dir_name = lines[i].split_whitespace().collect::<Vec<_>>()[2];
        match current_dir.get_subdir(new_dir_name) {
            Some(new_dir) => current_dir = new_dir,
            None => (),
        }
        i += 1;
    }

    let mut sizes = Vec::new();
    get_sizes(root_dir.clone(), &mut sizes);

    let sum: usize = sizes.iter().filter(|size| **size <= 100_000).sum();
    println!("Part 1: {}", sum);

    const TOTAL_SPACE: usize = 70000000;

    let available_space = TOTAL_SPACE - root_dir.size();
    let to_be_freed = 30000000 - available_space;

    let mut to_delete = sizes
        .iter()
        .filter(|size| **size >= to_be_freed)
        .collect::<Vec<_>>();

    to_delete.sort();
    let to_delete = to_delete[0];
    println!("Part 2: {}", to_delete);
}

fn get_sizes(dir: Rc<Directory>, sizes: &mut Vec<usize>) {
    sizes.push(dir.size());

    for subdir in dir.subdirs.borrow().iter() {
        get_sizes(subdir.clone(), sizes);
    }
}
