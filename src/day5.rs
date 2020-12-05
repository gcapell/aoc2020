use std::fs;
use std::io;
use std::io::BufRead;

pub fn highest_seat_id() {
    let x = io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(|s| Seat::new(&s).id())
        .max()
        .unwrap();
    println!("x:{}", x);
}

#[derive(Debug)]
struct Seat {
    row: i32,
    col: i32,
}

impl Seat {
    fn new(s: &str) -> Seat {
        Seat {
            row: binary(&s[..7], 'B'),
            col: binary(&s[7..], 'R'),
        }
    }
    fn id(&self) -> i32 {
        self.row * 8 + self.col
    }
}

fn binary(s: &str, one: char) -> i32 {
    s.chars()
        .map(|c| if c == one { 1 } else { 0 })
        .fold(0, |t, b| t * 2 + b)
}
