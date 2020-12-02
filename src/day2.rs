use std::fs;
use std::io::{self, BufRead};

pub fn count_matches(validate: fn(usize, usize, &str, &str) -> bool) -> usize {
    io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .filter(|x| {
            let (min, max, needle, haystack) = parse(x);
            validate(min, max, &needle[..1], haystack)
        })
        .count()
}

fn parse(line: &str) -> (usize, usize, &str, &str) {
    if let [counts, needle, haystack] = line
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .as_slice()
    {
        if let [min, max] = counts.split("-").collect::<Vec<&str>>().as_slice() {
            return (atou(min), atou(max), &needle[..1], haystack);
        }
    }
    panic!("bad line: {}", line);
}

fn atou(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}

pub fn policy1(min: usize, max: usize, needle: &str, haystack: &str) -> bool {
    let count = haystack.matches(needle).count();
    count >= min && count <= max
}

pub fn policy2(min: usize, max: usize, needle: &str, haystack: &str) -> bool {
    let needle = needle.chars().nth(0).unwrap();
    let a = haystack.chars().nth(min - 1).unwrap();
    let b = haystack.chars().nth(max - 1).unwrap();
    (a == needle) ^ (b == needle)
}
