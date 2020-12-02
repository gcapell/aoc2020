// use crate::util;
use std::fs;
use std::io::{self, BufRead, Error};


pub fn first() -> Result<(), Error> {
    let f = fs::File::open("input.txt")?;
    let lines = io::BufReader::new(f).lines();
    let mut count = 0;
    for line in lines {
        if let Ok(p) = line {
            if let [counts, needle, haystack]   = p.split_ascii_whitespace().collect::<Vec<&str>>().as_slice() {
                if let [ min, max ] = counts.split("-").collect::<Vec<&str>>().as_slice() {
                    let min = min.parse::<usize>().unwrap();
                    let max = max.parse::<usize>().unwrap();
                    if validate(min, max, &needle[..1], haystack) {
                        count += 1;
                    }
                    
                }
            }
        }
    }
    println!("count:{}", count);
    Ok(())
}

fn validate(min:usize, max:usize, needle:&str, haystack :&str)->bool {
  let count = haystack.matches(needle).count()   ;
    count >= min && count <= max
}
