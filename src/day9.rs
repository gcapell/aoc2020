use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;

pub fn run() {
    let nums = io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let target = find_missing_sum(&nums, 25).unwrap();
    println!("part1 {}", target);

    let v = contiguous_sum(&nums, target).unwrap();
    let part2 = v.iter().min().unwrap() + v.iter().max().unwrap();
    println!("part2 {}", part2);
}

fn find_missing_sum(v: &[i64], size: usize) -> Option<i64> {
    for j in size..v.len() {
        if !find_sum(v[j], &v[j - size..j]) {
            return Some(v[j]);
        }
    }
    None
}

fn find_sum(n: i64, v: &[i64]) -> bool {
    for (i, a) in v.iter().enumerate() {
        for b in v[i + 1..].iter() {
            if a + b == n {
                return true;
            }
        }
    }
    return false;
}

fn contiguous_sum(v: &[i64], target: i64) -> Option<&[i64]> {
    let mut totals = HashMap::new();
    let mut total = 0;
    for (i, n) in v.iter().enumerate() {
        total += n;
        totals.insert(total, i);
        if total < target {
            continue;
        }
        if let Some(j) = totals.get(&(total - target)) {
            return Some(&v[j + 1..i + 1]);
        }
    }
    None
}
