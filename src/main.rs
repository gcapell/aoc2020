use std::io::Error;
mod day1;
mod day2;
mod util;

fn main() -> Result<(), Error> {
    if false {
        day1::first(3)?;
    }
    day2::first()
    
}
