use std::fs;
use std::io;
use std::io::BufRead;

pub fn run() {
    let mut wx = 10;
    let mut wy = 1;
    let mut sx: i32 = 0;
    let mut sy: i32 = 0;
    for line in io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
    {
        match (&line[..1], line[1..].parse().unwrap()) {
            ("N", v) => wy += v,
            ("S", v) => wy -= v,
            ("E", v) => wx += v,
            ("W", v) => wx -= v,
            ("L", 90) | ("R", 270) => {
                let tmp = wy;
                wy = wx;
                wx = -tmp;
            }
            ("R", 90) | ("L", 270) => {
                let tmp = wy;
                wy = -wx;
                wx = tmp;
            }
            ("R", 180) | ("L", 180) => {
                wy = -wy;
                wx = -wx;
            }
            ("F", v) => {
                sx += wx * v;
                sy += wy * v;
            }
            (d, v) => panic!("d {}, v {}", d, v),
        }
    }
    println!("{}", sx.abs() + sy.abs());
}
