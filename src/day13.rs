use std::fs;
use std::io;
use std::io::BufRead;

pub fn run() {
    let mut f = io::BufReader::new(fs::File::open("input.txt").unwrap());
    let mut line = String::new();
    f.read_line(&mut line).unwrap();
    line.clear();
    f.read_line(&mut line).unwrap();

    let mut congruences = Vec::new();
    for (pos, bus) in line.trim().split(",").enumerate() {
        if let Ok(b) = bus.parse::<i64>() {
            let r = (b - ((pos as i64) % b)) % b;
            congruences.push((b, r));
        }
    }
    congruences.sort();
    congruences.reverse();
    println!("{}", solve_congruence(&congruences));
}

fn solve_congruence(mods: &[(i64, i64)]) -> i64 {
    let mut pos: i64 = mods[0].1;
    let mut add: i64 = mods[0].0;
    for (m, r) in &mods[1..] {
        while pos % m != *r {
            pos += add;
        }
        add *= m;
    }
    pos
}
