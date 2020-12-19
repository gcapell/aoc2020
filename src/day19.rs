use std::fs;
use std::io;
use std::io::BufRead;
use regex::Regex;

pub fn run() {
    let rules: Vec<Rule> = io::BufReader::new(fs::File::open("rules2.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(parse_rule)
        .collect();
    
    let pattern = format!("^{}$", regexp(&rules,0) );
    let c = Regex::new(&pattern).unwrap();
    
    let matches = io::BufReader::new(fs::File::open("input.txt").unwrap())
            .lines()
            .filter_map(Result::ok)
            .filter(|x| c.is_match(x))
            .count();    
    println!("{}",matches);
}

type Seq = Vec<usize>;

#[derive(Debug)]
enum Rule {
    Literal(char),
    Sequence(Seq),
    Alt(Seq, Seq),
}
use Rule::*;

fn regexp(rules: &Vec<Rule>, r:usize)->String{
    match &rules[r] {
        Literal(c)=>c.to_string(),
        Sequence(s)=> sregexp(rules,s),
        Alt(a,b)=> format!("({}|{})", sregexp(rules,a), sregexp(rules,b)),
    }
}

fn sregexp(rules:&Vec<Rule>, s:&Seq)->String{
    s.iter().map(|n| regexp(rules,*n)).collect()
}

fn parse_rule(line: String) -> Rule {
    if line.starts_with("\"") {
        return Literal(line.chars().skip(1).next().unwrap());
    }
    if let Some(p) = line.find("|") {
        return Alt(seq(&line[..p]), seq(&line[p + 1..]));
    }
    Sequence(seq(&line))
}

fn seq(line: &str) -> Seq {
    line.split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}
