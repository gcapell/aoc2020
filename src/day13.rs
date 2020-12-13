use std::fs;
use std::io;
use std::io::BufRead;


pub fn run()  {
    let mut f = io::BufReader::new(fs::File::open("input.txt").unwrap());
    let mut line = String::new();
    f.read_line(&mut line).unwrap();
    let start:i32 = line.trim().parse().unwrap();
    line.clear();
    f.read_line(&mut line).unwrap();
    println!("start {}", start);
    let mut min_wait = 0;
    let mut min_bus = 0;
    let mut first = true;
    for bus in line.trim().split(","){
        if let Ok(n) = bus.parse::<i32>() {
            let missed = start%n;
            let wait = if missed==0 {0} else {n-missed};
            if first || wait < min_wait {
                min_wait =wait;
                min_bus = n;
                first = false;
            }

            
         println!("bus {}, wait {}", n, wait);
         
        }
    }
    println!("min_bus {}, min_wait {}, product {}", min_bus, min_wait, min_bus*min_wait);

}
