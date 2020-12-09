use std::fs;
use std::io;
use std::io::BufRead;

pub fn run() {
    let nums = io::BufReader::new(fs::File::open("demo.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let size = 5;

    for j in size..nums.len() {
        if !find_sum(nums[j], &nums[j - size..j]) {
            println!("{}", nums[j]);
            break;
        }
    }
}

fn find_sum(n: i32, v: &[i32]) -> bool {
    for (i, a) in v.iter().enumerate() {
        for b in v[i + 1..].iter() {
            if a + b == n {
                return true;
            }
        }
    }
    return false;
}
