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

struct Lex<'a> {
    pos: usize,
    s: &'a [u8],
}

impl<'a> Lex<'a> {
    fn expect(&mut self, c: u8) -> bool {
        self.ws();
        if self.pos == self.s.len() {
            return false;
        }
        if self.s[self.pos] == c {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn pop(&mut self) -> u8 {
        self.ws();
        let c = self.s[self.pos];
        self.pos += 1;
        c
    }

    fn ws(&mut self) {
        while self.pos < self.s.len() && self.s[self.pos] == b' ' {
            self.pos += 1;
        }
    }
}

fn eval(line: String) -> i64 {
    let n = expr(&mut Lex {
        pos: 0,
        s: line.as_bytes(),
    });
    println!("{} = {}", line, n);
    n
}

fn expr(lex: &mut Lex) -> i64 {
    let mut acc = sum(lex);
    while lex.expect(b'*') {
        acc *= sum(lex);
    }
    acc
}

fn sum(lex: &mut Lex) -> i64 {
    let mut acc = term(lex);
    while lex.expect(b'+') {
        acc += term(lex);
    }
    acc
}

fn term(lex: &mut Lex) -> i64 {
    let c = lex.pop();
    if c == b'(' {
        let n = expr(lex);
        lex.expect(b')');
        n
    } else {
        (c - b'0').into()
    }
}
