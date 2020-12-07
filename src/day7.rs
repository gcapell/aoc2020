use std::fs;
use std::io;
use std::io::BufRead;
use regex::Regex;

pub fn dot() {
    let re = Regex::new(r"(\d+) (.*) bags?\.?").unwrap();
    println!("digraph G {{");
    for line in io::BufReader::new(fs::File::open("demo.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
    {
        if let [outer, inner] = line.split(" contain ").collect::<Vec<&str>>().as_slice() {
            let outer = outer.strip_suffix(" bags").unwrap();
            for x in inner.split(", ") {
                if x == "no other bags." {
                    continue;
                }
                if let Some(mat) = re.captures(x) {
                    println!(r#"   "{}"-> "{}" [ label = "{}"] "#, outer, &mat[2], &mat[1]);
                    } else {
                        println!("no match for {}", x);
                    }
                
            }
        }
    }
    println!("}}");
}
