use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug)]
struct Game {
    history: HashMap<i32, i32>,
    next: i32,
    turn: i32,
}

impl Game {
    fn store(&mut self, n: i32) {
        println!("store {}->{}", self.turn + 1, n);
        self.next = if let Some(h) = self.history.get(&n) {
            self.turn - h
        } else {
            0
        };
        self.history.insert(n, self.turn);
        self.turn += 1;
    }
}

pub fn run() {
    let start = Instant::now();
    let src = [0, 3, 1, 6, 7, 5];

    let mut g = Game {
        history: HashMap::new(),
        turn: 0,
        next: 0,
    };
    println!("src{:?},{:?}{:?}", src, g, start.elapsed());
    for n in &src {
        g.store(*n);
    }
    while g.turn < 2019 {
        g.store(g.next);
    }
    println!("{:?}", g.next);
}
