use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;

pub fn run() {
    let mut fixed = 0;
    let mut mem = HashMap::new();
    let mut floats = Vec::new();
    let mut z = 0;
    for line in io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
    {
        let chunks: Vec<&str> = line.split(" = ").collect();
        match chunks.as_slice() {
            ["mask", m] => {
                let (fi, fl) = parse_mask(m);
                fixed = fi;
                floats = fl;
                z = zeroes(&floats);
            }
            [addr, val] => {
                let addr = parse_addr(addr);
                let val = val.parse::<i64>().unwrap();
                let base = (addr | fixed) & !z;
                for m in 0..(1 << floats.len()) {
                    mem.insert(base | float_mask(m, &floats), val);
                }
            }
            _ => panic!(),
        }
    }
    println!("{}", mem.values().sum::<i64>())
}

fn float_mask(m: usize, floats: &[usize]) -> i64 {
    let mut reply = 0;
    for (i, p) in floats.iter().enumerate() {
        if m & (1 << i) != 0 {
            reply |= 1 << p;
        }
    }
    reply
}

fn zeroes(floats: &Vec<usize>) -> i64 {
    let mut z = 0;
    for u in floats {
        z |= 1 << u;
    }
    z
}

fn parse_mask(m: &str) -> (i64, Vec<usize>) {
    let mut fixed = 0;
    let mut float = Vec::new();
    for (p, c) in m.trim().chars().rev().enumerate() {
        match c {
            '0' => (),
            '1' => fixed |= 1 << p,
            'X' => float.push(p),
            _ => panic!(),
        }
    }
    (fixed, float)
}

fn parse_addr(m: &str) -> i64 {
    m[4..m.len() - 1].parse().unwrap()
}
