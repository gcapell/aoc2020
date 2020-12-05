use std::fs;
use std::io;
use std::io::BufRead;

pub fn highest_seat_id() {
    let x = io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(|line|id(&line))
        .max()
        .unwrap();
    println!("x:{}", x);
}

fn id(s: &str) -> i32 {
    binary(&s[..7], 'B') * 8 + binary(&s[7..], 'R')
}

fn binary(s: &str, one: char) -> i32 {
    s.chars()
        .map(|c| if c == one { 1 } else { 0 })
        .fold(0, |t, b| t * 2 + b)
}
