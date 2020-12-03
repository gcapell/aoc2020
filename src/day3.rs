use std::fs;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Square {
    Clear,
    Tree,
}

pub fn count_trees() {
    let course = io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().map(from_c).collect::<Vec<Square>>())
        .collect::<Vec<Vec<Square>>>();

    let mut h = 0;
    let mut count = 0;
    for line in course.iter() {
        if let Square::Tree = line[h] {
            count += 1;
        }
        h = (h + 3) % line.len()
    }
    println!(
        "course: {:?}, length:{}, trees:{}",
        course,
        course.len(),
        count
    );
}

fn from_c(c: char) -> Square {
    match c {
        '.' => Square::Clear,
        '#' => Square::Tree,
        _ => panic!("unexpected char {}", c),
    }
}
