use std::fs;
use std::io;
use std::io::BufRead;

pub fn run() {
    let instructions: Vec<Instruction> = io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(Instruction::parse)
        .collect();

    println!("{}", Machine::new().run_till_loop(instructions));
}

#[derive(Debug)]
enum Instruction {
    Nop,
    Acc(i32),
    Jmp(i32),
}
use Instruction::*;

impl Instruction {
    fn parse(s: String) -> Instruction {
        let code = &s[..3];
        let sign = if s.as_bytes()[4] == b'-' { -1 } else { 1 };
        let operand = &s[5..].parse::<i32>().unwrap() * sign;
        match code {
            "nop" => Nop,
            "acc" => Acc(operand),
            "jmp" => Jmp(operand),
            _ => panic!(s),
        }
    }
}

#[derive(Debug)]
struct Machine {
    pc: usize,
    acc: i32,
}

impl Machine {
    fn new() -> Machine {
        Machine { pc: 0, acc: 0 }
    }
    fn run_till_loop(self: &mut Machine, i: Vec<Instruction>) -> i32 {
        let mut executed = vec![false; i.len()];
        loop {
            if executed[self.pc] {
                return self.acc;
            }
            let prev = self.pc;
            match i[self.pc] {
                Nop => {
                    self.pc += 1;
                }
                Acc(n) => {
                    self.acc += n;
                    self.pc += 1;
                }
                Jmp(n) => {
                    self.pc = (self.pc as i32 + n) as usize;
                }
            }
            executed[prev] = true;
        }
    }
}
