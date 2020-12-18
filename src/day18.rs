use std::fs;
use std::io;
use std::io::BufRead;

pub fn run() {
    println!(
        "total: {}",
        io::BufReader::new(fs::File::open("input.txt").unwrap())
            .lines()
            .filter_map(Result::ok)
            .map(eval)
            .sum::<i64>()
    );
}

#[derive(Debug)]
struct Acc {
    n: Option<i64>,
    op: char,
}

impl Acc {
    fn new() -> Acc {
        Acc { n: None, op: ' ' }
    }

    fn merge(&mut self, v: i64) {
        self.n = if let Some(n) = self.n {
            Some(match self.op {
                '*' => v * n,
                '+' => v + n,
                _ => panic!(),
            })
        } else {
            Some(v)
        }
    }
}

fn eval(line: String) -> i64 {
    let mut acc = Acc::new();
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            ' ' => (),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                acc.merge(c as i64 - '0' as i64)
            }
            '(' => {
                stack.push(acc);
                acc = Acc::new();
            }
            ')' => {
                let p_acc = acc;
                acc = stack.pop().unwrap();
                acc.merge(p_acc.n.unwrap());
            }
            '*' | '+' => acc.op = c,
            _ => panic!(),
        }
        //println!("after {}, acc:{:?}, stack:{:?}", c, acc, stack);
    }
    let n = acc.n.unwrap();
    println!("{} = {}", line, n);
    n
}
