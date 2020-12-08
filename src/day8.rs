use std::fs;
use std::io;
use std::io::BufRead;

pub fn run() {
    let instructions: Vec<Instruction> = io::BufReader::new(fs::File::open("demo.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(Instruction::parse)
        .collect();
        println!("instructions:{:?}", instructions);

}

#[derive(Debug)]
enum Instruction {
    Nop,
    Acc(i32),
    Jmp(i32),
}
use Instruction::*;

impl Instruction {
    fn parse(s:String) -> Instruction {
        let code = &s[..3];
        let sign = if s.as_bytes()[4]  == b'-' { -1   } else {1}; 
        let operand = &s[5..].parse::<i32>().unwrap() * sign;
        match code {
            "nop" => Nop,
            "acc" => Acc(operand),
            "jmp" => Jmp(operand),
            _ => panic!(s),
        }
    }
}
