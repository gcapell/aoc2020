use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn count_valid() {
    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let contents = fs::read_to_string("input.txt").unwrap();
    let forms = kv_forms(&contents);
    let n = forms
        .iter()
        .filter(|form| required.iter().all(|field| valid(form, field)))
        .count();
    println!("valid:{}", n)
}

fn valid(form: &HashMap<&str, &str>, field: &str) -> bool {
    if !form.contains_key(field) {
        return false;
    }
    let v = form.get(field).unwrap();
    match field {
        "byr" => valid_year(v, 1920, 2002),
        "iyr" => valid_year(v, 2010, 2020),
        "eyr" => valid_year(v, 2020, 2030),
        "hgt" => measure(v, "cm", 150, 193) || measure(v, "in", 59, 76),
        "hcl" => Regex::new("^#[0-9a-f]{6}").unwrap().is_match(v),
        "ecl" => match *v {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        },
        "pid" => digits(v, 9),
        _ => true,
    }
}

fn measure(s: &str, suffix: &str, min: i32, max: i32) -> bool {
    match s.strip_suffix(suffix) {
        Some(n) => num_range(n, min, max),
        None => false,
    }
}

fn digits(s: &str, n: usize) -> bool {
    s.len() == n && s.chars().all(|c| c.is_ascii_digit())
}

fn num_range(n: &str, min: i32, max: i32) -> bool {
    match n.parse::<i32>() {
        Ok(i) => (i >= min && i <= max),
        Err(_) => false,
    }
}

fn valid_year(y: &str, min: i32, max: i32) -> bool {
    digits(y, 4) && num_range(y, min, max)
}

fn kv_forms(contents: &str) -> Vec<HashMap<&str, &str>> {
    contents
        .split("\n\n")
        .map(|para| {
            para.split_whitespace()
                .map(split)
                .collect::<HashMap<_, _>>()
        })
        .collect()
}

fn split(s: &str) -> (&str, &str) {
    if let Some(n) = s.find(':') {
        return (&s[..n], &s[n + 1..]);
    }
    panic!("could not split {}", s);
}
