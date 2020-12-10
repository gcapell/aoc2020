use std::collections::VecDeque;
use std::fs;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Adapter {
    value: i32,
    count: i64,
}

pub fn run() {
    let mut nums = io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    nums.sort();
    nums.insert(0, 0);
    nums.push(nums[nums.len() - 1] + 3);
    let mut ones = 0;
    let mut threes = 0;
    for v in nums.windows(2) {
        if let [a, b] = v {
            match b - a {
                1 => ones += 1,
                3 => threes += 1,
                _ => println!("{}-{}={}", b, a, b - a),
            }
        }
    }

    println!("{:?}, {}, {}, {}", nums, ones, threes, ones * threes);

    let mut q = VecDeque::with_capacity(3);
    q.push_front(Adapter { value: 0, count: 1 });
    for n in nums.iter().skip(1) {
        q.retain(|a|a.value+3 >= *n);
        q.push_front(Adapter {
            value: *n,
            count: q.iter().map(|a| a.count).sum(),
        });
    }
    println!("{:?}", q);
}
