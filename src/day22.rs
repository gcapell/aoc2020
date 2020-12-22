use std::collections::VecDeque;

type N = usize;
type Deck = VecDeque<N>;

pub fn run() {
    let demo = false;
    let mut a: Deck = VecDeque::from(vec![
        31, 24, 5, 33, 7, 12, 30, 22, 48, 14, 16, 26, 18, 45, 4, 42, 25, 20, 46, 21, 40, 38, 34,
        17, 50,
    ]);

    let mut b: Deck = VecDeque::from(vec![
        1, 3, 41, 8, 37, 35, 28, 39, 43, 29, 10, 27, 11, 36, 49, 32, 2, 23, 19, 9, 13, 15, 47, 6,
        44,
    ]);

    let winner = if game(&mut a, &mut b) { a } else { b };
    println!("score {}", score(&winner));
}

fn game(a: &mut Deck, b: &mut Deck) -> bool {
    let mut repeatchecker = RepeatChecker::new();
    while !a.is_empty() && !b.is_empty() {
        if repeatchecker.is_repeat(&a, &b) {
            return true;
        }
        let av = a.pop_front().unwrap();
        let bv = b.pop_front().unwrap();

        let a_wins = if av <= a.len() && bv <= b.len() {
            game(&mut sub_deck(a, av), &mut sub_deck(b, bv))
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

fn score(v: &Deck) -> N {
    v.iter().rev().enumerate().map(|(i, v)| (i + 1) * v).sum()
}

fn sub_deck(v: &Deck, n: N) -> Deck {
    let mut c = v.clone();
    c.truncate(n);
    c
}

struct RepeatChecker {
    seen: Vec<(N, N)>,
}

impl RepeatChecker {
    fn new() -> RepeatChecker {
        RepeatChecker { seen: Vec::new() }
    }

    fn is_repeat(&mut self, a: &Deck, b: &Deck) -> bool {
        let t = (score(a), score(b));
        self.seen.contains(&t) || {
            self.seen.push(t);
            false
        }
    }
}
