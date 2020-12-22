use std::collections::{VecDeque,HashSet};

type N = usize;

pub fn run() {
    let demo = false;
    let mut a: VecDeque<N> = VecDeque::from(if demo {vec![9, 2, 6, 3, 1]
        } else {vec![
        31, 24, 5, 33, 7, 12, 30, 22, 48, 14, 16, 26, 18, 45, 4, 42, 25, 20, 46, 21, 40, 38, 34,
        17, 50,
    ]});

    let mut b: VecDeque<N> = VecDeque::from(if demo {vec![5, 8, 4, 7, 10]} else {vec![
        1, 3, 41, 8, 37, 35, 28, 39, 43, 29, 10, 27, 11, 36, 49, 32, 2, 23, 19, 9, 13, 15, 47, 6,
        44,
    ]});
 

    let winner = if game(&mut a, &mut b, 0) { a } else { b };

    let score: N = winner
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) * v)
        .sum();
    println!("score {}", score);
}

struct RepeatChecker {
    seen: HashSet<String>,
}

impl RepeatChecker {
    fn new() -> RepeatChecker {
        RepeatChecker { seen: HashSet::new() }
    }

    fn is_repeat(&mut self, a: &VecDeque<N>, b: &VecDeque<N>) -> bool {
        let s = format!("{:?}{:?}",a,b);
        if self.seen.contains(&s) {
            true
        } else {
            self.seen.insert(s);
            false            
        }
    }
}

fn game(a: &mut VecDeque<N>, b: &mut VecDeque<N>, depth: usize) -> bool {
    let mut repeatchecker = RepeatChecker::new();
    while !a.is_empty() && !b.is_empty() {
        if true {
            println!();
            println!(
                "{dummy:>depth$}{a:?}",
                dummy = "#",
                depth = depth * 3,
                a = a
            );
            println!(
                "{dummy:>depth$}{b:?}",
                dummy = "#",
                depth = depth * 3,
                b = b
            );
        }
        if  repeatchecker.is_repeat(&a, &b) {
            return true;
        }
        let av = a.pop_front().unwrap();
        let bv = b.pop_front().unwrap();

        let a_wins = if av <= a.len() && bv <= b.len() {
            let mut c = a.clone();
            c.truncate(av);
            let mut d = b.clone();
            d.truncate(bv);
            game(&mut c, &mut d, depth + 1)
        } else {
            av > bv
        };

        if a_wins {
            a.push_back(av);
            a.push_back(bv);
        } else {
            b.push_back(bv);
            b.push_back(av);
        }
    }
    b.is_empty()
}
