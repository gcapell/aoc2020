
use std::fs;
use std::io;
use std::io::BufRead;


pub fn run() {
    let mut x = 0;
    let mut y = 0;
    let mut heading = 90; // north is 0
    for line in io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(Result::ok) {
            let d = &line[..1];
            let v:i32 = line[1..].parse().unwrap();
            println!("{}:{}",d,v);
            match d{
                "N" => y += v,
                "S" => y -= v,
                "E" => x += v,
                "W" => x -= v,
                "L" => heading = mod360(heading -v),
                "R" => heading = mod360(heading +v),
                "F" => match heading {
                    0 => y += v,
                    180 => y -= v,
                    90 => x += v,
                    270 => x -= v,
                    _ => panic!("heading{}",heading),
                }
                _ => panic!("d {}",d),
            }
            println!("{},{}({})",x,y,heading);
        }
        println!("{}", x.abs() + y.abs());
}

fn mod360(mut h:i32)->i32 {
    h %=360;
    if h<0 {h+360} else {h}
}