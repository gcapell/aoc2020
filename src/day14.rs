use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;

pub fn run() {
    let mut fixed = 0;
    let mut mask = 0;
    let mut mem = HashMap::new();
    for line in io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
    {
        let chunks: Vec<&str> = line.split(" = ").collect();
        match chunks.as_slice() {
            ["mask", m] => {
                let (f, m) = parse_mask(m);
                fixed = f;
                mask = m;
            }
            [pos, val] => {
                let pos = parse_pos(pos);
                let src = val.parse::<i64>().unwrap();
                mem.insert(pos, fixed | (src & mask));
            }
            _ => panic!(),
        }
    }
    println!("map: {:?}", mem);
    println!("{}", mem.values().sum::<i64>())
}

fn parse_mask(m: &str) -> (i64, i64) {
    let mut fixed = 0;
    let mut mask = 0;
    for (p, c) in m.trim().chars().rev().enumerate() {
        match c {
            '0' => (),
            '1' => fixed |= 1 << p,
            'X' => mask |= 1 << p,
            _ => panic!(),
        }
    }
    println!("parse_mask({})->fixed:{:b}, mask:{:b}", m, fixed, mask);
    (fixed, mask)
}

fn parse_pos(m: &str) -> i64 {
    m[4..m.len() - 1].parse().unwrap()
}
