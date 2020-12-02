use crate::util;
use std::fs;
use std::io::Error;

pub fn first(size: usize) -> Result<(), Error> {
    let mut v = util::ints_from(fs::File::open("input.txt")?)?;
    v.sort();

    let mut answer = vec![0; size];
    if util::summing_elements(&v, size, 2020, &mut answer) {
        println!("{:?}-> {}", answer, answer.iter().fold(1, |acc, x| acc * x));
    }
    Ok(())
}
