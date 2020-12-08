use std::fs;
use std::io;
use std::io::BufRead;

pub fn run() {
    let instructions: Vec<Instruction> = io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(Instruction::parse)
        .collect();

    let mut i_swapped = vec![false; instructions.len()];
    loop {
        if let Some(n) = Machine::new().run_till_loop_or_end(&instructions, &mut i_swapped) {
            println!("{}", n);
            break;
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Nop(i32),
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
            "nop" => Nop(operand),
            "acc" => Acc(operand),
            "jmp" => Jmp(operand),
            _ => panic!(s),
        }
    }
    fn swap(self: Instruction) -> Option<Instruction> {
        match self {
            Nop(n) => Some(Jmp(n)),
            Acc(_) => None,
            Jmp(n) => Some(Nop(n)),
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
    fn run_till_loop_or_end(
        self: &mut Machine,
        instructions: &Vec<Instruction>,
        i_swapped: &mut Vec<bool>,
    ) -> Option<i32> {
        let mut executed = vec![false; instructions.len()];
        let mut swapped = false;

        loop {
            if self.pc == instructions.len() {
                return Some(self.acc);
            }
            if executed[self.pc] {
                return None;
            }
            let mut i = instructions[self.pc];
            if !swapped && !i_swapped[self.pc] {
                if let Some(s) = i.swap() {
                    i = s;
                    swapped = true;
                    i_swapped[self.pc] = true;
                }
            }
            let prev = self.pc;
            match i {
                Nop(_) => {
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
