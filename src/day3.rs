use std::fs;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Square {
    Clear,
    Tree,
}

#[derive(Debug)]
struct Slope {
    right: usize,
    down: usize,
}

pub fn count_trees() {
    let course = io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().map(from_c).collect::<Vec<Square>>())
        .collect::<Vec<Vec<Square>>>();

    let slopes = vec![
        Slope { right: 3, down: 1 },
        Slope { right: 1, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];
    let product = slopes
        .iter()
        .map(|x| trees(&course, x))
        .fold(1, |p, x| p * x);
    println!("product {}", product);
}

fn trees(course: &Vec<Vec<Square>>, s: &Slope) -> i64 {
    let mut row = 0;
    let mut col = 0;
    let mut count = 0;
    while row < course.len() {
        let line = &course[row];
        row += s.down;
        if let Square::Tree = line[col] {
            count += 1;
        }
        col = (col + s.right) % line.len();
    }
    println!("count{}", count);
    count
}

fn from_c(c: char) -> Square {
    match c {
        '.' => Square::Clear,
        '#' => Square::Tree,
        _ => panic!("unexpected char {}", c),
    }
}
