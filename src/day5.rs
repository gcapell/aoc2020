use std::fs;
use std::io;
use std::io::BufRead;

pub fn highest_seat_id() {
    let mut ids: Vec<i32> = io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(binary)
        .collect();
    ids.sort();
    println!(
        "missing: {:?}",
        &ids.windows(2)
            .find_map(|w| match w {
                &[a,b] if a+1 != b => Some(a+1),
                _ => None,
            })
    );
}

fn binary(s: String) -> i32 {
    s.chars()
        .map(|c|  (c == 'B' || c == 'R') as i32 )
        .fold(0, |t, b| t * 2 + b)
}
