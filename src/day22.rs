use std::collections::VecDeque;

type N = usize;

pub fn run() {
    let mut a: VecDeque<N> = VecDeque::new();
    let mut b: VecDeque<N> = VecDeque::new();
    for i in [
        31, 24, 5, 33, 7, 12, 30, 22, 48, 14, 16, 26, 18, 45, 4, 42, 25, 20, 46, 21, 40, 38, 34,
        17, 50,
    ]
    .iter()
    {
        a.push_back(*i);
    }
    for i in [
        1, 3, 41, 8, 37, 35, 28, 39, 43, 29, 10, 27, 11, 36, 49, 32, 2, 23, 19, 9, 13, 15, 47, 6,
        44,
    ]
    .iter()
    {
        b.push_back(*i);
    }
    while !a.is_empty() && !b.is_empty() {
        println!("{:?}, {:?}", a, b);
        let av = a.pop_front().unwrap();
        let bv = b.pop_front().unwrap();
        if av > bv {
            a.push_back(av);
            a.push_back(bv);
        } else {
            b.push_back(bv);
            b.push_back(av);
        }
    }
    let winner = if a.is_empty() { b } else { a };
    let score: N = winner
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) * v)
        .sum();
    println!("score {}", score);
}
