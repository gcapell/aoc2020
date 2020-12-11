use std::fs;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
enum Pos {
    Floor,
    Empty,
    Occupied,
}
use Pos::*;

pub fn run() {
    let ferry = io::BufReader::new(fs::File::open("demo.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(row)
        .collect::<Vec<Vec<Pos>>>();
    out(&ferry);
}

fn out(v: &Vec<Vec<Pos>>) {
    for r in v.iter() {
        println!(
            "{}",
            r.iter()
                .map(|p| match p {
                    Floor => '.',
                    Empty => 'L',
                    Occupied => '#',
                })
                .collect::<String>()
        );
    }
}

fn row(s: String) -> Vec<Pos> {
    s.chars()
        .map(|c| match c {
            'L' => Empty,
            '.' => Floor,
            '#' => Occupied,
            _ => panic!(),
        })
        .collect()
}
