use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Cube {
    neighbours: i32,
}

type Point = (i32, i32, i32);

pub fn run() {
    let mut active = HashSet::new();
    for (y, line) in io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .enumerate()
    {
        println!("{}:{}", y, line);
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active.insert((x as i32, y as i32, 0));
            }
        }
    }
    for _ in 0..6 {
        active = cycle(active);
    }
    println!("{}", active.len());
}

fn cycle(active: HashSet<Point>) -> HashSet<Point> {
    let mut created: HashMap<Point, i32> = HashMap::new();
    let mut existing: HashMap<Point, i32> = HashMap::new();

    for p in active.iter() {
        for q in MyRange::new(*p) {
            let m = if active.contains(&q) {
                &mut existing
            } else {
                &mut created
            };
            *m.entry(q).or_insert(0) += 1;
        }
    }

    let mut reply: HashSet<Point> = HashSet::new();
    for (p, c) in created.iter() {
        if *c == 3 {
            reply.insert(*p);
        }
    }
    for (p, c) in existing.iter() {
        if *c == 2 || *c == 3 {
            reply.insert(*p);
        }
    }
    reply
}

struct MyRange {
    p: Point,
    dx: i32,
    dy: i32,
    dz: i32,
    done: bool,
}

impl MyRange {
    fn new(p: Point) -> MyRange {
        MyRange {
            p: p,
            dx: -1,
            dy: -1,
            dz: -1,
            done: false,
        }
    }
}

impl Iterator for MyRange {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.done {
            return None;
        }
        let reply = (self.p.0 + self.dx, self.p.1 + self.dy, self.p.2 + self.dz);
        loop {
            self.dx += 1;
            if self.dx == 2 {
                self.dx = -1;
                self.dy += 1;
            }
            if self.dy == 2 {
                self.dy = -1;
                self.dz += 1;
            }
            if self.dz == 2 {
                self.done = true;
            }
            if !(self.dx == 0 && self.dy == 0 && self.dz == 0) {
                break;
            }
        }
        Some(reply)
    }
}