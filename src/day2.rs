use std::fs;
use std::io::{self, BufRead};

pub fn first() {
    println!("count:{}", count_matches(policy1));
}
pub fn second() {
    println!("count:{}", count_matches(policy2));
}

fn count_matches(validate: fn(usize, usize, &str, &str) -> bool) -> i32 {
    let f = fs::File::open("input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let mut count = 0;
    for line in lines {
        if let Ok(p) = line {
            if let [counts, needle, haystack] =
                p.split_ascii_whitespace().collect::<Vec<&str>>().as_slice()
            {
                if let [min, max] = counts.split("-").collect::<Vec<&str>>().as_slice() {
                    let min = min.parse::<usize>().unwrap();
                    let max = max.parse::<usize>().unwrap();
                    if validate(min, max, &needle[..1], haystack) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn policy1(min: usize, max: usize, needle: &str, haystack: &str) -> bool {
    let count = haystack.matches(needle).count();
    count >= min && count <= max
}

fn policy2(min: usize, max: usize, needle: &str, haystack: &str) -> bool {
    let needle = needle.chars().nth(0).unwrap();
    let a = haystack.chars().nth(min - 1).unwrap();
    let b = haystack.chars().nth(max - 1).unwrap();
    (a == needle) ^ (b == needle)
}
