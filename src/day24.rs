use std::collections::{HashMap, HashSet};
use std::fs;

pub fn run() {
    let origin = Pos { e: 0, ne: 0 };
    let mut black = HashSet::new();
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let dirs = parse(&line);
        let dst = origin.traverse(&dirs);
        if !black.insert(dst) {
            black.remove(&dst);
        }
        //println!("LINE {:?} -> {:?}", dirs, dst);
    }
    println!("black: {}", black.len());
    for _day in 1..=100 {
        black = life(&black);
        //println!("Day {}: {}", day, black.len());
    }
    println!("{}", black.len())
}

fn life(board: &HashSet<Pos>) -> HashSet<Pos> {
    let mut neighbours: HashMap<Pos, i32> = HashMap::new();
    for t in board {
        for n in t.neighbours() {
            *neighbours.entry(n).or_insert(0) += 1;
        }
    }
    //println!("neighbours {:?}",neighbours);

    let mut reply: HashSet<Pos> = HashSet::new();
    // still black?
    for p in board {
        match neighbours.get(p) {
            Some(&n) if n == 1 || n == 2 => reply.insert(*p),
            _ => false,
        };
    }

    // new black?
    for (p, &n) in neighbours.iter() {
        if n == 2 && !board.contains(p) {
            reply.insert(*p);
        }
    }
    reply
}

#[derive(Debug)]
enum Direction {
    E,
    W,
    NE,
    SE,
    NW,
    SW,
}
use Direction::*;

type PN = i32;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    e: PN,
    ne: PN,
}

impl Pos {
    fn traverse(&self, ds: &Vec<Direction>) -> Pos {
        let Pos { mut e, mut ne } = self;
        for d in ds {
            match d {
                E => e += 1,
                W => e -= 1,
                NE => ne += 1,
                SW => ne -= 1,
                SE => {
                    e += 1;
                    ne -= 1
                }
                NW => {
                    e -= 1;
                    ne += 1
                }
            }
        }
        Pos { e: e, ne: ne }
    }
    fn neighbours(&self) -> Vec<Pos> {
        vec![
            Pos {
                e: self.e + 1,
                ne: self.ne,
            },
            Pos {
                e: self.e - 1,
                ne: self.ne,
            },
            Pos {
                e: self.e,
                ne: self.ne + 1,
            },
            Pos {
                e: self.e,
                ne: self.ne - 1,
            },
            Pos {
                e: self.e + 1,
                ne: self.ne - 1,
            },
            Pos {
                e: self.e - 1,
                ne: self.ne + 1,
            },
        ]
    }
}

fn parse(s: &str) -> Vec<Direction> {
    let mut v = Vec::new();
    let mut cs = s.chars();
    while let Some(c) = cs.next() {
        v.push(match c {
            'e' => E,
            'w' => W,
            'n' => match cs.next() {
                Some('e') => NE,
                Some('w') => NW,
                _ => panic!(),
            },
            's' => match cs.next() {
                Some('e') => SE,
                Some('w') => SW,
                _ => panic!(),
            },
            _ => panic!(),
        });
    }
    v
}
