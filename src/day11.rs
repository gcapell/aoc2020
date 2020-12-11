use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pos {
    Floor,
    Empty,
    Occupied,
}
use Pos::*;

type Board = Vec<Vec<Pos>>;
type Point = (usize, usize);

pub fn run() {
    let mut a = io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(row)
        .collect::<Vec<Vec<Pos>>>();
    //out(&a);

    let start = Instant::now();
    let n = neighbours(&a);

    let mut b = dummy(&a);

    loop {
        let diff = step(&a, &mut b, &n);
        //out(&b);
        if !diff {
            break;
        }
        let c = b;
        b = a;
        a = c;
    }
    let occupied = a.iter()
            .map(|v| v.iter().filter(|p| **p == Occupied).count())
            .sum::<usize>();
    println!("{:?}", start.elapsed());
    //out(&a);
    println!( "{}", occupied);

}

fn neighbours(v: &Board) -> HashMap<Point, Vec<Point>> {
    let mut reply = HashMap::new();
    for (i, row) in v.iter().enumerate() {
        for (j, s) in row.iter().enumerate() {
            if *s == Floor {
                continue;
            }
            let p = (i, j);
            reply.insert(p, point_neighbours(v, p));
        }
    }
    reply
}

fn point_neighbours(v: &Board, o: Point) -> Vec<Point> {
    let height = v.len();
    let width = v[0].len();
    let mut reply = Vec::new();
    for f in [nw, n, ne, e, se, s, sw, west].iter() {
        let mut p = o;
        loop {
            if let Some(n) = f(p, height - 1, width - 1) {
                p = n;
                if v[p.0][p.1] != Floor {
                    reply.push(p);
                    break;
                }
            } else {
                break;
            }
        }
    }
    reply
}

fn nw(p: Point, _h: usize, _w: usize) -> Option<Point> {
    if p.0 == 0 || p.1 == 0 {
        None
    } else {
        Some((p.0 - 1, p.1 - 1))
    }
}

fn n(p: Point, _h: usize, _w: usize) -> Option<Point> {
    if p.0 == 0 {
        None
    } else {
        Some((p.0 - 1, p.1))
    }
}

fn ne(p: Point, _h: usize, w: usize) -> Option<Point> {
    if p.0 == 0 || p.1 == w {
        None
    } else {
        Some((p.0 - 1, p.1 + 1))
    }
}

fn e(p: Point, _h: usize, w: usize) -> Option<Point> {
    if p.1 == w {
        None
    } else {
        Some((p.0, p.1 + 1))
    }
}

fn se(p: Point, h: usize, w: usize) -> Option<Point> {
    if p.0 == h || p.1 == w {
        None
    } else {
        Some((p.0 + 1, p.1 + 1))
    }
}
fn s(p: Point, h: usize, _w: usize) -> Option<Point> {
    if p.0 == h {
        None
    } else {
        Some((p.0 + 1, p.1))
    }
}
fn sw(p: Point, h: usize, _w: usize) -> Option<Point> {
    if p.0 == h || p.1 == 0 {
        None
    } else {
        Some((p.0 + 1, p.1 - 1))
    }
}
fn west(p: Point, _h: usize, _w: usize) -> Option<Point> {
    if p.1 == 0 {
        None
    } else {
        Some((p.0, p.1 - 1))
    }
}

fn step(src: &Board, dst: &mut Board, neighbours: &HashMap<Point, Vec<Point>>) -> bool {
    let mut change = false;
    for (p, ns) in neighbours.iter() {
        dst[p.0][p.1] = match src[p.0][p.1] {
            Floor => Floor,
            Empty => {
                if unoccupied(src, ns) {
                    change = true;
                    Occupied
                } else {
                    Empty
                }
            }
            Occupied => {
                if crowded(src, ns) {
                    change = true;
                    Empty
                } else {
                    Occupied
                }
            }
        };
    }

    change
}

fn unoccupied(b: &Board, v: &Vec<Point>) -> bool {
    !v.iter().any(|p| b[p.0][p.1] == Occupied)
}

fn crowded(b: &Board, v: &Vec<Point>) -> bool {
    v.iter()
        .filter(|p| b[p.0][p.1] == Occupied)
        .skip(4)
        .next()
        .is_some()
}

fn dummy(v: &Board) -> Board {
    let mut reply = Vec::new();
    for r in v {
        reply.push(vec![Floor; r.len()]);
    }
    reply
}

fn out(v: &Board) {
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
    println!("");
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
