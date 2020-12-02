use std::fs;
use std::io::Error;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub fn first(size: usize) -> Result<(), Error> {
    let mut v = ints_from(fs::File::open("input.txt")?)?;
    v.sort();

    let mut answer = vec![0; size];
    if summing_elements(&v, size, 2020, &mut answer) {
        println!("{:?}-> {}", answer, answer.iter().fold(1, |acc, x| acc * x));
    }
    Ok(())
}

pub fn summing_elements(v: &[i64], elems: usize, target: i64, answer: &mut [i64]) -> bool {
    if elems == 1 {
        if let Ok(pos) = v.binary_search(&target) {
            answer[0] = v[pos];
            return true;
        } else {
            return false;
        }
    }

    for (i, k) in v.iter().enumerate() {
        if k > &target {
            return false;
        }
        answer[0] = *k;
        if summing_elements(&v[i + 1..], elems - 1, target - k, &mut answer[1..]) {
            return true;
        }
    }
    return false;
}

pub fn ints_from<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}


