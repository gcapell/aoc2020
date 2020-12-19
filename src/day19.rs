use regex::Regex;
use std::fs;
use std::io;
use std::io::BufRead;

pub fn run() {
    let rules: Vec<Rule> = io::BufReader::new(fs::File::open("rules2.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(parse_rule)
        .collect();

    let a = regex(&rules, 42);
    let b = regex(&rules, 31);

    let matches = io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .filter(|x| more_matches(&a, &b, x))
        .count();
    println!("{}", matches);
}

type Seq = Vec<usize>;

#[derive(Debug)]
enum Rule {
    Literal(char),
    Sequence(Seq),
    Alt(Seq, Seq),
}
use Rule::*;

fn more_matches(a: &regex::Regex, b: &regex::Regex, s: &str) -> bool {
    let (na, t) = count_matches(a, s);
    if na < 2 {
        return false;
    }
    let (nb, u) = count_matches(b, t);
    nb > 0 && nb < na && u.is_empty()
}

fn count_matches<'a>(r: &'a regex::Regex, s: &'a str) -> (i32, &'a str) {
    let mut count = 0;
    let mut t = s;
    while let Some(m) = r.find(t) {
        count += 1;
        t = &t[m.end()..]
    }
    if count > 0 {
        println!("count_matches ->{},{}", count, t);
    }
    (count, t)
}

fn regex(rules: &Vec<Rule>, r: usize) -> regex::Regex {
    let p = format!("^{}", pattern(rules, r));
    Regex::new(&p).unwrap()
}

fn pattern(rules: &Vec<Rule>, r: usize) -> String {
    match &rules[r] {
        Literal(c) => c.to_string(),
        Sequence(s) => spattern(rules, s),
        Alt(a, b) => format!("({}|{})", spattern(rules, a), spattern(rules, b)),
    }
}

fn spattern(rules: &Vec<Rule>, s: &Seq) -> String {
    s.iter().map(|n| pattern(rules, *n)).collect()
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
